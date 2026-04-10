(** * GridType: ARC Grid as Dependent Function Type

    Formalizes the encoding of ARC grids as type-theoretic terms
    and proves basic properties of grid transformations.
*)

From Stdlib Require Import Arith List Bool PeanoNat Lia.
Import ListNotations.

(** ** Color type: natural numbers in [0,9] *)

Definition Color := nat.
Definition valid_color (c : Color) : Prop := c < 10.

(** ** Grid as a flat data vector with dimensions *)

Record Grid := mkGrid {
  grid_rows : nat;
  grid_cols : nat;
  grid_data : list Color
}.

Definition grid_wf (g : Grid) : Prop :=
  length (grid_data g) = grid_rows g * grid_cols g.

Definition grid_valid (g : Grid) : Prop :=
  Forall (fun c => c < 10) (grid_data g).

(** ** Grid access *)

Definition grid_get (g : Grid) (r c : nat) : option Color :=
  if (r <? grid_rows g) && (c <? grid_cols g)
  then nth_error (grid_data g) (r * grid_cols g + c)
  else None.

(** ** A transformation maps grid data to grid data *)

Definition Transform := list Color -> list Color.

Definition correct_on (f : Transform) (input output : list Color) : Prop :=
  f input = output.

(** ** Correctness on examples *)

Theorem verified_transform_correct :
  forall (f : Transform) (examples : list (list Color * list Color)),
  Forall (fun p => correct_on f (fst p) (snd p)) examples ->
  forall input output,
    In (input, output) examples ->
    f input = output.
Proof.
  intros f examples Hcorr input output Hin.
  rewrite Forall_forall in Hcorr.
  specialize (Hcorr _ Hin). simpl in Hcorr.
  exact Hcorr.
Qed.

(** ** Identity transform *)

Definition identity_transform : Transform := fun data => data.

Theorem identity_correct :
  forall examples,
  Forall (fun p => fst p = snd p) examples ->
  Forall (fun p => correct_on identity_transform (fst p) (snd p)) examples.
Proof.
  intros examples H.
  rewrite Forall_forall in *.
  intros x Hin. unfold correct_on, identity_transform.
  apply H. exact Hin.
Qed.

(** ** Color mapping preserves length *)

Definition color_map_fn (m : Color -> Color) : Transform :=
  fun data => map m data.

Lemma color_map_preserves_length :
  forall (m : Color -> Color) (data : list Color),
  length (color_map_fn m data) = length data.
Proof.
  intros. unfold color_map_fn. apply length_map.
Qed.

(** ** Spatial transform properties *)

(** Flip (reverse) is an involution *)
Definition flip_data (data : list Color) : list Color := rev data.

Lemma flip_involution : forall data, flip_data (flip_data data) = data.
Proof. intros. unfold flip_data. apply rev_involutive. Qed.

Lemma flip_preserves_length : forall data,
  length (flip_data data) = length data.
Proof. intros. unfold flip_data. apply length_rev. Qed.

(** ** Color map composition *)

Lemma color_map_compose : forall (f g : Color -> Color) (data : list Color),
  map f (map g data) = map (fun x => f (g x)) data.
Proof. intros. rewrite map_map. reflexivity. Qed.

(** ** Bijective color maps are invertible *)

Theorem bijective_color_map_invertible :
  forall (m m_inv : Color -> Color),
  (forall x, m_inv (m x) = x) ->
  forall data,
  color_map_fn m_inv (color_map_fn m data) = data.
Proof.
  intros m m_inv Hinv data.
  unfold color_map_fn.
  rewrite map_map.
  induction data as [|x xs IH].
  - reflexivity.
  - simpl. rewrite Hinv. f_equal. exact IH.
Qed.

(** ** Valid color preservation under mapping *)

Lemma color_map_preserves_valid :
  forall (m : Color -> Color) (data : list Color),
  (forall c, c < 10 -> m c < 10) ->
  Forall (fun c => c < 10) data ->
  Forall (fun c => c < 10) (map m data).
Proof.
  intros m data Hm Hvalid.
  rewrite Forall_forall in *.
  intros x Hin.
  apply in_map_iff in Hin.
  destruct Hin as [y [Heq Hy]].
  subst. apply Hm. apply Hvalid. exact Hy.
Qed.

(** ** NbE idempotence (axiomatized — proved in Rust) *)

Axiom nbe_normalize : list Color -> list Color.
Axiom nbe_idempotent : forall data,
  nbe_normalize (nbe_normalize data) = nbe_normalize data.

Theorem grid_nbe_stable :
  forall data,
  nbe_normalize (nbe_normalize data) = nbe_normalize data.
Proof. intros. apply nbe_idempotent. Qed.

(** ** NbE equality is an equivalence relation *)

Definition nbe_equal (a b : list Color) : Prop :=
  nbe_normalize a = nbe_normalize b.

Theorem nbe_equal_refl : forall a, nbe_equal a a.
Proof. intros. unfold nbe_equal. reflexivity. Qed.

Theorem nbe_equal_sym : forall a b, nbe_equal a b -> nbe_equal b a.
Proof. intros. unfold nbe_equal in *. symmetry. exact H. Qed.

Theorem nbe_equal_trans : forall a b c,
  nbe_equal a b -> nbe_equal b c -> nbe_equal a c.
Proof. intros. unfold nbe_equal in *. rewrite H. exact H0. Qed.
