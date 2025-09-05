//! Collaborative Editing and Proof Sharing
//!
//! Real-time collaboration features:
//! - Operational Transformation for concurrent editing
//! - WebSocket-based synchronization
//! - Presence awareness
//! - Proof sharing and forking

use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebSocket, MessageEvent, CloseEvent, ErrorEvent};
use gloo_timers::callback::Interval;

/// Collaborative editor component
#[derive(Properties, PartialEq)]
pub struct CollaborativeEditorProps {
    pub session_id: String,
}

#[function_component(CollaborativeEditor)]
pub fn collaborative_editor(props: &CollaborativeEditorProps) -> Html {
    let session = use_state(|| Session::new(&props.session_id));
    let websocket = use_state(|| None::<WebSocket>);
    let local_version = use_state(|| 0u64);
    let pending_ops = use_state(|| VecDeque::<Operation>::new());
    
    // Connect to collaboration server
    use_effect_with(props.session_id.clone(), {
        let websocket = websocket.clone();
        let session = session.clone();
        
        move |session_id| {
            let ws_url = format!("wss://sctt.example.com/collaborate/{}", session_id);
            
            match WebSocket::new(&ws_url) {
                Ok(ws) => {
                    // Set up message handler
                    let session = session.clone();
                    let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
                        if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
                            let msg: ServerMessage = serde_json::from_str(&text.as_string().unwrap()).unwrap();
                            handle_server_message(msg, &session);
                        }
                    }) as Box<dyn FnMut(MessageEvent)>);
                    
                    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                    onmessage.forget();
                    
                    websocket.set(Some(ws));
                }
                Err(e) => {
                    web_sys::console::error_1(&JsValue::from_str(&format!("WebSocket error: {:?}", e)));
                }
            }
            
            || () // Cleanup
        }
    });
    
    let on_edit = {
        let session = session.clone();
        let websocket = websocket.clone();
        let local_version = local_version.clone();
        let pending_ops = pending_ops.clone();
        
        Callback::from(move |op: Operation| {
            // Apply operation locally
            apply_operation(&session, &op);
            
            // Queue for sending
            let mut ops = (*pending_ops).clone();
            ops.push_back(op.clone());
            pending_ops.set(ops);
            
            // Send to server
            if let Some(ws) = &*websocket {
                let msg = ClientMessage::Operation {
                    op,
                    version: *local_version,
                };
                
                if let Ok(json) = serde_json::to_string(&msg) {
                    let _ = ws.send_with_str(&json);
                }
            }
            
            local_version.set(*local_version + 1);
        })
    };
    
    html! {
        <div class="collaborative-editor">
            <div class="session-header">
                <h2>{format!("Session: {}", props.session_id)}</h2>
                <div class="users">
                    {session.users.iter().map(|user| {
                        html! {
                            <UserAvatar user={user.clone()} />
                        }
                    }).collect::<Html>()}
                </div>
                <button class="btn" onclick={share_session}>{"Share"}</button>
            </div>
            
            <div class="editor-area">
                <CollaborativeCodeEditor 
                    document={session.document.clone()}
                    on_edit={on_edit}
                    cursors={get_cursor_positions(&session)}
                />
                
                <div class="proof-panel">
                    <ProofCollaboration 
                        proof_state={session.proof_state.clone()}
                    />
                </div>
            </div>
            
            <div class="collab-tools">
                <button class="btn">{"Fork"}</button>
                <button class="btn">{"Export"}</button>
                <button class="btn">{"History"}</button>
                <button class="btn">{"Comments"}</button>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct UserAvatarProps {
    user: User,
}

#[function_component(UserAvatar)]
fn user_avatar(props: &UserAvatarProps) -> Html {
    html! {
        <div class="user-avatar" style={format!("border-color: {}", props.user.color)}>
            <span class="avatar-letter">{props.user.name.chars().next().unwrap_or('?')}</span>
            <span class="user-name">{&props.user.name}</span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CollaborativeCodeEditorProps {
    document: Document,
    on_edit: Callback<Operation>,
    cursors: Vec<CursorInfo>,
}

#[function_component(CollaborativeCodeEditor)]
fn collaborative_code_editor(props: &CollaborativeCodeEditorProps) -> Html {
    let content = use_state(|| props.document.content.clone());
    
    let on_input = {
        let on_edit = props.on_edit.clone();
        let content = content.clone();
        
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let textarea = target.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
            let new_content = textarea.value();
            
            // Calculate operation
            let op = calculate_operation(&content, &new_content);
            on_edit.emit(op);
            
            content.set(new_content);
        })
    };
    
    html! {
        <div class="collab-code-editor">
            <textarea 
                value={(*content).clone()}
                oninput={on_input}
                class="code-area"
            />
            
            {props.cursors.iter().map(|cursor| {
                html! {
                    <div 
                        class="remote-cursor"
                        style={format!(
                            "left: {}px; top: {}px; background-color: {}",
                            cursor.x, cursor.y, cursor.color
                        )}
                    >
                        <span class="cursor-label">{&cursor.user_name}</span>
                    </div>
                }
            }).collect::<Html>()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ProofCollaborationProps {
    proof_state: ProofState,
}

#[function_component(ProofCollaboration)]
fn proof_collaboration(props: &ProofCollaborationProps) -> Html {
    html! {
        <div class="proof-collaboration">
            <h3>{"Collaborative Proof"}</h3>
            
            <div class="proof-goals">
                {props.proof_state.goals.iter().map(|goal| {
                    html! {
                        <div class="goal-card">
                            <div class="goal-header">
                                {format!("Goal {}", goal.id)}
                            </div>
                            <div class="goal-content">
                                {&goal.conclusion}
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
            
            <div class="proof-history">
                <h4>{"Recent Steps"}</h4>
                {props.proof_state.history.iter().rev().take(5).map(|step| {
                    html! {
                        <div class="proof-step">
                            <span class="tactic">{&step.tactic}</span>
                            <span class="author">{"by user"}</span>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}

/// Operational Transformation implementation
pub struct OperationalTransform;

impl OperationalTransform {
    /// Transform operation A against operation B
    pub fn transform(a: &Operation, b: &Operation) -> (Operation, Operation) {
        match (a, b) {
            (Operation::Insert { pos: pos_a, text: text_a }, 
             Operation::Insert { pos: pos_b, text: text_b }) => {
                if pos_a < pos_b {
                    (a.clone(), Operation::Insert {
                        pos: pos_b + text_a.len(),
                        text: text_b.clone(),
                    })
                } else if pos_a > pos_b {
                    (Operation::Insert {
                        pos: pos_a + text_b.len(),
                        text: text_a.clone(),
                    }, b.clone())
                } else {
                    // Same position - use deterministic ordering
                    if text_a < text_b {
                        (a.clone(), Operation::Insert {
                            pos: pos_b + text_a.len(),
                            text: text_b.clone(),
                        })
                    } else {
                        (Operation::Insert {
                            pos: pos_a + text_b.len(),
                            text: text_a.clone(),
                        }, b.clone())
                    }
                }
            }
            
            (Operation::Delete { pos: pos_a, len: len_a },
             Operation::Delete { pos: pos_b, len: len_b }) => {
                if pos_a + len_a <= *pos_b {
                    (a.clone(), Operation::Delete {
                        pos: pos_b - len_a,
                        len: *len_b,
                    })
                } else if pos_b + len_b <= *pos_a {
                    (Operation::Delete {
                        pos: pos_a - len_b,
                        len: *len_a,
                    }, b.clone())
                } else {
                    // Overlapping deletes
                    let start = (*pos_a).min(*pos_b);
                    let end = (pos_a + len_a).max(pos_b + len_b);
                    let total_len = end - start;
                    
                    (Operation::Delete {
                        pos: start,
                        len: total_len - len_b,
                    }, Operation::Delete {
                        pos: start,
                        len: total_len - len_a,
                    })
                }
            }
            
            (Operation::Insert { pos: pos_a, text },
             Operation::Delete { pos: pos_b, len }) => {
                if pos_a <= pos_b {
                    (a.clone(), Operation::Delete {
                        pos: pos_b + text.len(),
                        len: *len,
                    })
                } else if pos_a >= pos_b + len {
                    (Operation::Insert {
                        pos: pos_a - len,
                        text: text.clone(),
                    }, b.clone())
                } else {
                    // Insert within delete range
                    (Operation::Insert {
                        pos: *pos_b,
                        text: text.clone(),
                    }, Operation::Delete {
                        pos: *pos_b,
                        len: len + text.len(),
                    })
                }
            }
            
            (Operation::Delete { .. }, Operation::Insert { .. }) => {
                let (b_prime, a_prime) = Self::transform(b, a);
                (a_prime, b_prime)
            }
            
            _ => (a.clone(), b.clone()),
        }
    }
    
    /// Apply operation to document
    pub fn apply(doc: &mut String, op: &Operation) {
        match op {
            Operation::Insert { pos, text } => {
                doc.insert_str(*pos, text);
            }
            Operation::Delete { pos, len } => {
                doc.drain(*pos..*pos + len);
            }
            Operation::Replace { pos, len, text } => {
                doc.drain(*pos..*pos + len);
                doc.insert_str(*pos, text);
            }
        }
    }
}

/// Server messages
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Welcome {
        session: Session,
        user_id: String,
    },
    UserJoined {
        user: User,
    },
    UserLeft {
        user_id: String,
    },
    Operation {
        op: Operation,
        user_id: String,
        version: u64,
    },
    CursorUpdate {
        user_id: String,
        cursor: CursorPosition,
    },
    ProofUpdate {
        proof_state: ProofState,
    },
    Error {
        message: String,
    },
}

/// Client messages
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Join {
        user_name: String,
    },
    Operation {
        op: Operation,
        version: u64,
    },
    CursorUpdate {
        cursor: CursorPosition,
    },
    ProofAction {
        action: ProofAction,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProofAction {
    ApplyTactic {
        goal_id: usize,
        tactic: String,
    },
    Undo,
    Redo,
}

/// Cursor information for display
#[derive(Clone, PartialEq)]
struct CursorInfo {
    user_name: String,
    x: i32,
    y: i32,
    color: String,
}

// Helper functions

fn handle_server_message(msg: ServerMessage, session: &UseStateHandle<Session>) {
    match msg {
        ServerMessage::Operation { op, .. } => {
            apply_operation(session, &op);
        }
        ServerMessage::UserJoined { user } => {
            let mut s = (**session).clone();
            s.users.push(user);
            session.set(s);
        }
        ServerMessage::UserLeft { user_id } => {
            let mut s = (**session).clone();
            s.users.retain(|u| u.id != user_id);
            session.set(s);
        }
        _ => {}
    }
}

fn apply_operation(session: &UseStateHandle<Session>, op: &Operation) {
    let mut s = (**session).clone();
    OperationalTransform::apply(&mut s.document.content, op);
    s.document.version += 1;
    s.document.operations.push(op.clone());
    session.set(s);
}

fn calculate_operation(old: &str, new: &str) -> Operation {
    // Simplified diff - would use proper diff algorithm
    if new.len() > old.len() {
        Operation::Insert {
            pos: old.len(),
            text: new[old.len()..].to_string(),
        }
    } else if new.len() < old.len() {
        Operation::Delete {
            pos: new.len(),
            len: old.len() - new.len(),
        }
    } else {
        Operation::Replace {
            pos: 0,
            len: old.len(),
            text: new.to_string(),
        }
    }
}

fn get_cursor_positions(session: &Session) -> Vec<CursorInfo> {
    session.users.iter().map(|user| {
        CursorInfo {
            user_name: user.name.clone(),
            x: (user.cursor.column * 8) as i32,
            y: (user.cursor.line * 20) as i32,
            color: user.color.clone(),
        }
    }).collect()
}

fn share_session(_: MouseEvent) {
    // Copy session URL to clipboard
    if let Some(window) = web_sys::window() {
        if let Some(location) = window.location().href().ok() {
            let _ = window.navigator().clipboard().write_text(&location);
        }
    }
}

use crate::{Session, User, Document, Operation, CursorPosition, ProofState};

impl Session {
    pub fn new(id: &str) -> Self {
        Session {
            id: id.to_string(),
            users: Vec::new(),
            document: Document {
                content: String::new(),
                version: 0,
                operations: Vec::new(),
            },
            proof_state: ProofState {
                goals: Vec::new(),
                history: Vec::new(),
            },
        }
    }
}