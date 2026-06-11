(** Data Generation Pipeline for ARC Training
    Converts puzzle solving into training data with cognitive traces *)

open Core
open Arc.Pattern_engine
open Arc.Solver

(** Training data types *)
type training_example = {
  puzzle_id: string;
  input: grid;
  output: grid;
  solution_path: solution_step list;
  cognitive_trace: cognitive_trace;
  metadata: training_metadata;
}

and solution_step = {
  step_number: int;
  state: grid;
  action: action;
  reasoning: string;
  confidence: float;
  time_taken: float;
}

and action =
  | SelectCell of int * int
  | ApplyTransform of transformation
  | ComposeTransforms of transformation list
  | Backtrack of int (* steps to backtrack *)
  | Explore of exploration_type
  | Verify of verification_type

and exploration_type =
  | SymmetryExploration
  | PatternSearch
  | RuleDiscovery
  | AnalogicalReasoning

and verification_type =
  | InvariantCheck
  | ConsistencyCheck
  | CompletionCheck

and cognitive_trace = {
  attention_sequence: attention_point list;
  working_memory: working_memory_state list;
  insight_moments: insight list;
  error_corrections: error_correction list;
  strategy_switches: strategy_switch list;
}

and attention_point = {
  timestamp: float;
  focus: focus_region;
  saliency: float;
  purpose: attention_purpose;
}

and focus_region =
  | Cell of int * int
  | Region of int * int * int * int
  | Pattern of sub_pattern
  | Global

and attention_purpose =
  | Scanning
  | Comparing
  | Verifying
  | Planning

and working_memory_state = {
  timestamp: float;
  active_patterns: pattern list;
  active_rules: RuleInference.rule list;
  hypothesis: string option;
  confidence: float;
}

and insight = {
  timestamp: float;
  discovery: discovery_type;
  impact: float;
  explanation: string;
}

and discovery_type =
  | PatternRecognition of pattern
  | RuleInference of RuleInference.rule
  | SymmetryDetection of symmetry
  | AnalogicalMapping of pattern * pattern

and error_correction = {
  timestamp: float;
  error_type: error_type;
  correction: action;
  learning: string;
}

and error_type =
  | MisappliedRule
  | IncorrectPattern
  | OverGeneralization
  | UnderGeneralization

and strategy_switch = {
  timestamp: float;
  from_strategy: solving_strategy;
  to_strategy: solving_strategy;
  reason: string;
}

and solving_strategy =
  | TrialAndError
  | SystematicSearch
  | PatternMatching
  | AbstractReasoning
  | AnalogicalTransfer

and training_metadata = {
  difficulty: float;
  solution_quality: float;
  novelty: float;
  generalization_potential: float;
  cognitive_load: float;
}

(** Cognitive simulation *)
module CognitiveSimulator = struct
  type simulator_state = {
    current_grid: grid;
    target_grid: grid;
    steps_taken: solution_step list;
    attention_history: attention_point list;
    working_memory: working_memory_state;
    current_strategy: solving_strategy;
    start_time: float;
  }
  
  (** Simulate human-like problem solving *)
  let simulate_solving (input: grid) (output: grid) : solution_step list * cognitive_trace =
    let initial_state = {
      current_grid = input;
      target_grid = output;
      steps_taken = [];
      attention_history = [];
      working_memory = {
        timestamp = 0.0;
        active_patterns = [];
        active_rules = [];
        hypothesis = None;
        confidence = 0.0;
      };
      current_strategy = PatternMatching;
      start_time = 0.0;
    } in
    
    let final_state = solve_with_cognition initial_state in
    
    let trace = {
      attention_sequence = final_state.attention_history;
      working_memory = [final_state.working_memory];
      insight_moments = extract_insights final_state.steps_taken;
      error_corrections = extract_corrections final_state.steps_taken;
      strategy_switches = extract_switches final_state.steps_taken;
    } in
    
    (final_state.steps_taken, trace)
  
  and solve_with_cognition state =
    if grid_equal state.current_grid state.target_grid then
      state
    else if List.length state.steps_taken > 100 then
      state (* Timeout *)
    else
      let next_action = decide_next_action state in
      let new_state = apply_cognitive_action state next_action in
      solve_with_cognition new_state
  
  and grid_equal g1 g2 =
    Array.for_all2_exn g1 g2 ~f:(fun r1 r2 ->
      Array.for_all2_exn r1 r2 ~f:(=))
  
  and decide_next_action state =
    (* Simulate cognitive decision making *)
    match state.current_strategy with
    | PatternMatching ->
        (* Look for patterns *)
        let patterns = detect_salient_patterns state.current_grid in
        if List.is_empty patterns then
          Explore PatternSearch
        else
          let transform = pattern_to_transform (List.hd_exn patterns) in
          ApplyTransform transform
    
    | SystematicSearch ->
        (* Try transformations systematically *)
        let next_transform = get_next_systematic_transform state in
        ApplyTransform next_transform
    
    | AbstractReasoning ->
        (* Use abstract reasoning *)
        let reasoning = abstract_reason state in
        match reasoning with
        | Some action -> action
        | None -> Explore RuleDiscovery
    
    | AnalogicalTransfer ->
        (* Find analogies *)
        Explore AnalogicalReasoning
    
    | TrialAndError ->
        (* Random exploration *)
        let random_transform = generate_random_transform () in
        ApplyTransform random_transform
  
  and detect_salient_patterns grid =
    let features = Analysis.extract_features grid in
    List.map features.shapes ~f:(fun shape ->
      {
        input = grid;
        output = grid;
        metadata = {
          symmetries = features.symmetries;
          transformations = [];
          invariants = [];
          complexity = features.complexity;
        };
      })
  
  and pattern_to_transform pattern =
    match pattern.metadata.symmetries with
    | Horizontal :: _ -> Geometric (Flip `Horizontal)
    | Vertical :: _ -> Geometric (Flip `Vertical)
    | Rotational n :: _ -> Geometric (Rotate n)
    | _ -> ColorMap [(0, 1)]
  
  and get_next_systematic_transform state =
    let all_transforms = [
      Geometric (Rotate 90);
      Geometric (Rotate 180);
      Geometric (Rotate 270);
      Geometric (Flip `Horizontal);
      Geometric (Flip `Vertical);
    ] in
    
    let tried = List.filter_map state.steps_taken ~f:(fun step ->
      match step.action with
      | ApplyTransform t -> Some t
      | _ -> None) in
    
    List.find all_transforms ~f:(fun t ->
      not (List.mem tried t ~equal:transformation_equal))
    |> Option.value ~default:(ColorMap [(0, 1)])
  
  and transformation_equal t1 t2 =
    match t1, t2 with
    | Geometric g1, Geometric g2 -> geometric_equal g1 g2
    | ColorMap m1, ColorMap m2 -> m1 = m2
    | _ -> false
  
  and geometric_equal g1 g2 =
    match g1, g2 with
    | Rotate n1, Rotate n2 -> n1 = n2
    | Flip d1, Flip d2 -> d1 = d2
    | _ -> false
  
  and abstract_reason state =
    if Random.float 1.0 < 0.3 then
      Some (Verify InvariantCheck)
    else
      None
  
  and generate_random_transform () =
    if Random.bool () then
      Geometric (Rotate (Random.int 4 * 90))
    else
      ColorMap [(Random.int 10, Random.int 10)]
  
  and apply_cognitive_action state action =
    let timestamp = state.start_time +. float_of_int (List.length state.steps_taken) in
    
    (* Update attention *)
    let attention = generate_attention action timestamp in
    
    (* Apply action *)
    let new_grid = match action with
      | ApplyTransform t -> 
          RuleInference.apply_transformation t state.current_grid
      | _ -> state.current_grid in
    
    (* Create step *)
    let step = {
      step_number = List.length state.steps_taken + 1;
      state = new_grid;
      action = action;
      reasoning = explain_action action;
      confidence = compute_confidence state action;
      time_taken = Random.float 2.0 +. 1.0;
    } in
    
    (* Update state *)
    {
      state with
      current_grid = new_grid;
      steps_taken = step :: state.steps_taken;
      attention_history = attention :: state.attention_history;
      working_memory = update_working_memory state.working_memory action;
    }
  
  and generate_attention action timestamp =
    {
      timestamp;
      focus = (match action with
        | SelectCell (i, j) -> Cell (i, j)
        | _ -> Global);
      saliency = Random.float 1.0;
      purpose = (match action with
        | Verify _ -> Verifying
        | Explore _ -> Scanning
        | _ -> Planning);
    }
  
  and explain_action action =
    match action with
    | ApplyTransform t -> "Applying transformation"
    | SelectCell (i, j) -> Printf.sprintf "Selecting cell (%d, %d)" i j
    | Explore _ -> "Exploring possibilities"
    | Verify _ -> "Verifying solution"
    | _ -> "Thinking..."
  
  and compute_confidence state action =
    0.5 +. Random.float 0.5
  
  and update_working_memory mem action =
    { mem with timestamp = mem.timestamp +. 1.0 }
  
  and extract_insights steps =
    List.filter_map steps ~f:(fun step ->
      if step.confidence > 0.8 then
        Some {
          timestamp = step.time_taken;
          discovery = PatternRecognition {
            input = step.state;
            output = step.state;
            metadata = {
              symmetries = [];
              transformations = [];
              invariants = [];
              complexity = 0.0;
            };
          };
          impact = step.confidence;
          explanation = step.reasoning;
        }
      else None)
  
  and extract_corrections steps =
    [] (* Simplified *)
  
  and extract_switches steps =
    [] (* Simplified *)
end

(** Data augmentation *)
module Augmentation = struct
  (** Generate variations of training examples *)
  let augment_example (ex: training_example) : training_example list =
    let rotations = generate_rotations ex in
    let reflections = generate_reflections ex in
    let color_permutations = generate_color_permutations ex in
    let noise_additions = add_noise ex in
    
    ex :: (rotations @ reflections @ color_permutations @ noise_additions)
  
  and generate_rotations ex =
    List.map [90; 180; 270] ~f:(fun angle ->
      let rotated_input = rotate_grid ex.input angle in
      let rotated_output = rotate_grid ex.output angle in
      { ex with 
        input = rotated_input; 
        output = rotated_output;
        puzzle_id = ex.puzzle_id ^ "_rot" ^ Int.to_string angle;
      })
  
  and rotate_grid grid angle =
    match angle with
    | 90 -> rotate_90 grid
    | 180 -> rotate_90 (rotate_90 grid)
    | 270 -> rotate_90 (rotate_90 (rotate_90 grid))
    | _ -> grid
  
  and rotate_90 grid =
    let h = Array.length grid in
    let w = if h > 0 then Array.length grid.(0) else 0 in
    Array.init w ~f:(fun j ->
      Array.init h ~f:(fun i ->
        grid.(h - 1 - i).(j)))
  
  and generate_reflections ex =
    [
      { ex with
        input = flip_horizontal ex.input;
        output = flip_horizontal ex.output;
        puzzle_id = ex.puzzle_id ^ "_flip_h";
      };
      { ex with
        input = flip_vertical ex.input;
        output = flip_vertical ex.output;
        puzzle_id = ex.puzzle_id ^ "_flip_v";
      };
    ]
  
  and flip_horizontal grid =
    Array.map grid ~f:Array.rev
  
  and flip_vertical grid =
    Array.rev grid
  
  and generate_color_permutations ex =
    let perm = generate_random_permutation () in
    [{
      ex with
      input = apply_color_permutation ex.input perm;
      output = apply_color_permutation ex.output perm;
      puzzle_id = ex.puzzle_id ^ "_color_perm";
    }]
  
  and generate_random_permutation () =
    let colors = Array.init 10 ~f:(fun i -> i) in
    Array.permute colors;
    colors
  
  and apply_color_permutation grid perm =
    Array.map grid ~f:(fun row ->
      Array.map row ~f:(fun c ->
        if c >= 0 && c < 10 then perm.(c) else c))
  
  and add_noise ex =
    if Random.float 1.0 < 0.2 then
      [{
        ex with
        input = add_noise_to_grid ex.input 0.05;
        puzzle_id = ex.puzzle_id ^ "_noise";
      }]
    else []
  
  and add_noise_to_grid grid prob =
    Array.map grid ~f:(fun row ->
      Array.map row ~f:(fun c ->
        if Random.float 1.0 < prob then
          Random.int 10
        else c))
end

(** Format conversion *)
module FormatConversion = struct
  (** Convert to JSON format *)
  let to_json (ex: training_example) : Yojson.Basic.t =
    `Assoc [
      ("puzzle_id", `String ex.puzzle_id);
      ("input", grid_to_json ex.input);
      ("output", grid_to_json ex.output);
      ("solution_path", solution_path_to_json ex.solution_path);
      ("cognitive_trace", cognitive_trace_to_json ex.cognitive_trace);
      ("metadata", metadata_to_json ex.metadata);
    ]
  
  and grid_to_json grid =
    `List (List.map (Array.to_list grid) ~f:(fun row ->
      `List (List.map (Array.to_list row) ~f:(fun c -> `Int c))))
  
  and solution_path_to_json steps =
    `List (List.map steps ~f:step_to_json)
  
  and step_to_json step =
    `Assoc [
      ("step_number", `Int step.step_number);
      ("state", grid_to_json step.state);
      ("action", action_to_json step.action);
      ("reasoning", `String step.reasoning);
      ("confidence", `Float step.confidence);
      ("time_taken", `Float step.time_taken);
    ]
  
  and action_to_json action =
    match action with
    | SelectCell (i, j) -> 
        `Assoc [("type", `String "select"); ("i", `Int i); ("j", `Int j)]
    | ApplyTransform _ -> 
        `Assoc [("type", `String "transform")]
    | _ -> 
        `Assoc [("type", `String "other")]
  
  and cognitive_trace_to_json trace =
    `Assoc [
      ("attention_sequence", `List (List.map trace.attention_sequence ~f:attention_to_json));
      ("working_memory", `List (List.map trace.working_memory ~f:memory_to_json));
      ("insight_moments", `List (List.map trace.insight_moments ~f:insight_to_json));
      ("error_corrections", `List (List.map trace.error_corrections ~f:correction_to_json));
      ("strategy_switches", `List (List.map trace.strategy_switches ~f:switch_to_json));
    ]
  
  and attention_to_json att =
    `Assoc [
      ("timestamp", `Float att.timestamp);
      ("saliency", `Float att.saliency);
    ]
  
  and memory_to_json mem =
    `Assoc [
      ("timestamp", `Float mem.timestamp);
      ("confidence", `Float mem.confidence);
    ]
  
  and insight_to_json ins =
    `Assoc [
      ("timestamp", `Float ins.timestamp);
      ("impact", `Float ins.impact);
      ("explanation", `String ins.explanation);
    ]
  
  and correction_to_json corr =
    `Assoc [
      ("timestamp", `Float corr.timestamp);
      ("learning", `String corr.learning);
    ]
  
  and switch_to_json switch =
    `Assoc [
      ("timestamp", `Float switch.timestamp);
      ("reason", `String switch.reason);
    ]
  
  and metadata_to_json meta =
    `Assoc [
      ("difficulty", `Float meta.difficulty);
      ("solution_quality", `Float meta.solution_quality);
      ("novelty", `Float meta.novelty);
      ("generalization_potential", `Float meta.generalization_potential);
      ("cognitive_load", `Float meta.cognitive_load);
    ]
  
  (** Convert to tensor format for ML *)
  let to_tensor (ex: training_example) : float array array =
    let input_flat = flatten_grid ex.input in
    let output_flat = flatten_grid ex.output in
    let features = extract_numeric_features ex in
    
    [|
      Array.concat [input_flat; output_flat; features];
    |]
  
  and flatten_grid grid =
    Array.concat (Array.to_list (Array.map grid ~f:(fun row ->
      Array.map row ~f:float_of_int)))
  
  and extract_numeric_features ex =
    [|
      ex.metadata.difficulty;
      ex.metadata.solution_quality;
      ex.metadata.novelty;
      ex.metadata.generalization_potential;
      ex.metadata.cognitive_load;
      float_of_int (List.length ex.solution_path);
      float_of_int (List.length ex.cognitive_trace.insight_moments);
    |]
end

(** Pipeline orchestration *)
module Pipeline = struct
  type pipeline_config = {
    augmentation_factor: int;
    include_cognitive_trace: bool;
    output_format: output_format;
    quality_threshold: float;
    batch_size: int;
  }
  
  and output_format =
    | JSON
    | Tensor
    | Both
  
  (** Process single puzzle *)
  let process_puzzle (config: pipeline_config) (puzzle: pattern) : training_example list =
    (* Simulate solving *)
    let (steps, trace) = CognitiveSimulator.simulate_solving puzzle.input puzzle.output in
    
    (* Create base example *)
    let base_example = {
      puzzle_id = generate_puzzle_id ();
      input = puzzle.input;
      output = puzzle.output;
      solution_path = steps;
      cognitive_trace = if config.include_cognitive_trace then trace 
                       else empty_trace ();
      metadata = compute_metadata puzzle steps trace;
    } in
    
    (* Augment if configured *)
    let examples = 
      if config.augmentation_factor > 1 then
        Augmentation.augment_example base_example
        |> List.take config.augmentation_factor
      else [base_example] in
    
    (* Filter by quality *)
    List.filter examples ~f:(fun ex ->
      ex.metadata.solution_quality >= config.quality_threshold)
  
  and generate_puzzle_id () =
    "puzzle_" ^ Int.to_string (Random.int 1000000)
  
  and empty_trace () =
    {
      attention_sequence = [];
      working_memory = [];
      insight_moments = [];
      error_corrections = [];
      strategy_switches = [];
    }
  
  and compute_metadata puzzle steps trace =
    {
      difficulty = estimate_difficulty puzzle;
      solution_quality = evaluate_solution_quality steps;
      novelty = compute_novelty puzzle;
      generalization_potential = estimate_generalization puzzle;
      cognitive_load = compute_cognitive_load trace;
    }
  
  and estimate_difficulty puzzle =
    Analysis.compute_complexity puzzle.input
  
  and evaluate_solution_quality steps =
    let efficiency = 1.0 /. (1.0 +. float_of_int (List.length steps)) in
    let confidence = List.fold steps ~init:0.0 ~f:(fun acc s ->
      acc +. s.confidence) /. float_of_int (List.length steps) in
    (efficiency +. confidence) /. 2.0
  
  and compute_novelty puzzle =
    Random.float 1.0 (* Simplified *)
  
  and estimate_generalization puzzle =
    0.5 +. Random.float 0.5
  
  and compute_cognitive_load trace =
    let switches = float_of_int (List.length trace.strategy_switches) in
    let corrections = float_of_int (List.length trace.error_corrections) in
    (switches +. corrections) /. 10.0
  
  (** Process batch of puzzles *)
  let process_batch (config: pipeline_config) (puzzles: pattern list) : unit =
    let all_examples = List.concat_map puzzles ~f:(process_puzzle config) in
    
    (* Group into batches *)
    let batches = chunk_list all_examples config.batch_size in
    
    (* Save each batch *)
    List.iteri batches ~f:(fun i batch ->
      save_batch config batch i)
  
  and chunk_list lst size =
    let rec chunk acc current remaining =
      match remaining with
      | [] -> 
          if List.is_empty current then acc 
          else List.rev current :: acc
      | h :: t ->
          if List.length current >= size then
            chunk (List.rev current :: acc) [h] t
          else
            chunk acc (h :: current) t
    in
    List.rev (chunk [] [] lst)
  
  and save_batch config batch batch_num =
    match config.output_format with
    | JSON -> save_json_batch batch batch_num
    | Tensor -> save_tensor_batch batch batch_num
    | Both -> 
        save_json_batch batch batch_num;
        save_tensor_batch batch batch_num
  
  and save_json_batch batch num =
    let json_list = List.map batch ~f:FormatConversion.to_json in
    let json = `List json_list in
    let filename = Printf.sprintf "training_batch_%d.json" num in
    Yojson.Basic.to_file filename json
  
  and save_tensor_batch batch num =
    let tensors = List.concat_map batch ~f:(fun ex ->
      Array.to_list (FormatConversion.to_tensor ex)) in
    let filename = Printf.sprintf "training_batch_%d.npy" num in
    (* Would save as numpy array *)
    ()
end

(** Public API *)
let default_config = {
  Pipeline.augmentation_factor = 4;
  include_cognitive_trace = true;
  output_format = Pipeline.Both;
  quality_threshold = 0.6;
  batch_size = 32;
}

let generate_training_data = Pipeline.process_batch
let process_single = Pipeline.process_puzzle
let augment = Augmentation.augment_example
let to_json = FormatConversion.to_json
let to_tensor = FormatConversion.to_tensor