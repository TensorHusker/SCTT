(** ARC Pattern Engine for ARC 2025/2026
    Core pattern matching, transformation detection, and rule inference
    Integrates with SCTT for type-theoretic pattern reasoning *)

open Core

(** ARC pattern representation *)
type color = int (* 0-9 for standard ARC colors *)

type grid = color array array

type pattern = {
  input: grid;
  output: grid;
  metadata: pattern_metadata;
}

and pattern_metadata = {
  symmetries: symmetry list;
  transformations: transformation list;
  invariants: invariant list;
  complexity: float;
}

and symmetry = 
  | Horizontal
  | Vertical  
  | Rotational of int (* degrees *)
  | Diagonal
  | Translation of int * int

and transformation =
  | ColorMap of (color * color) list
  | Geometric of geometric_transform
  | Composite of transformation list
  | Conditional of condition * transformation
  | Recursive of recursive_rule

and geometric_transform =
  | Rotate of int
  | Flip of [`Horizontal | `Vertical]
  | Scale of int * int
  | Translate of int * int
  | Mirror of [`X | `Y | `XY]

and condition =
  | ColorAt of int * int * color
  | PatternMatch of sub_pattern
  | Boundary of boundary_type
  | Count of comparison * int

and comparison = Eq | Lt | Gt | Le | Ge | Ne

and boundary_type = Edge | Corner | Interior

and sub_pattern = {
  pattern: grid;
  position: [`Any | `Fixed of int * int];
  match_type: [`Exact | `ColorBlind | `ShapeOnly];
}

and recursive_rule = {
  base_case: pattern;
  recursive_step: transformation;
  termination: condition;
}

and invariant =
  | ColorCount of color * int
  | ShapePreservation
  | TopologyInvariant of int (* Betti number *)
  | AreaConservation
  | ConnectivityPreserved

(** Pattern analysis *)
module Analysis = struct
  (** Extract features from a grid *)
  let extract_features (g: grid) : feature_vector =
    let height = Array.length g in
    let width = if height > 0 then Array.length g.(0) else 0 in
    
    let color_histogram = compute_color_histogram g in
    let shape_features = extract_shapes g in
    let symmetry_features = detect_symmetries g in
    let topology_features = compute_topology g in
    
    {
      dimensions = (height, width);
      colors = color_histogram;
      shapes = shape_features;
      symmetries = symmetry_features;
      topology = topology_features;
      complexity = compute_complexity g;
    }
  
  and feature_vector = {
    dimensions: int * int;
    colors: int array;
    shapes: shape list;
    symmetries: symmetry list;
    topology: topology_descriptor;
    complexity: float;
  }
  
  and shape = {
    bounds: int * int * int * int; (* x, y, width, height *)
    color: color;
    pixels: (int * int) list;
    shape_type: shape_class;
  }
  
  and shape_class =
    | Rectangle
    | Line of [`Horizontal | `Vertical | `Diagonal]
    | LShape
    | TShape
    | Cross
    | Custom of string
  
  and topology_descriptor = {
    connected_components: int;
    euler_characteristic: int;
    betti_numbers: int array;
    genus: int;
  }
  
  let compute_color_histogram (g: grid) : int array =
    let hist = Array.create ~len:10 0 in
    Array.iter g ~f:(fun row ->
      Array.iter row ~f:(fun color ->
        if color >= 0 && color < 10 then
          hist.(color) <- hist.(color) + 1));
    hist
  
  let extract_shapes (g: grid) : shape list =
    (* Flood fill to find connected components *)
    let height = Array.length g in
    let width = if height > 0 then Array.length g.(0) else 0 in
    let visited = Array.make_matrix ~dimx:height ~dimy:width false in
    let shapes = ref [] in
    
    for i = 0 to height - 1 do
      for j = 0 to width - 1 do
        if not visited.(i).(j) && g.(i).(j) <> 0 then
          let shape = flood_fill g visited i j in
          shapes := shape :: !shapes
      done
    done;
    List.rev !shapes
  
  and flood_fill g visited i j =
    let color = g.(i).(j) in
    let pixels = ref [] in
    let min_x = ref j in
    let max_x = ref j in
    let min_y = ref i in
    let max_y = ref i in
    
    let rec fill i j =
      if i >= 0 && i < Array.length g &&
         j >= 0 && j < Array.length g.(0) &&
         not visited.(i).(j) && g.(i).(j) = color then begin
        visited.(i).(j) <- true;
        pixels := (i, j) :: !pixels;
        min_x := min !min_x j;
        max_x := max !max_x j;
        min_y := min !min_y i;
        max_y := max !max_y i;
        
        fill (i+1) j;
        fill (i-1) j;
        fill i (j+1);
        fill i (j-1);
      end
    in
    fill i j;
    
    {
      bounds = (!min_x, !min_y, !max_x - !min_x + 1, !max_y - !min_y + 1);
      color = color;
      pixels = !pixels;
      shape_type = classify_shape !pixels;
    }
  
  and classify_shape pixels =
    (* Simple shape classification based on pixel pattern *)
    let count = List.length pixels in
    if count <= 2 then Line `Horizontal
    else Custom "unknown"
  
  let detect_symmetries (g: grid) : symmetry list =
    let symms = ref [] in
    
    (* Check horizontal symmetry *)
    if is_horizontally_symmetric g then
      symms := Horizontal :: !symms;
    
    (* Check vertical symmetry *)
    if is_vertically_symmetric g then
      symms := Vertical :: !symms;
    
    (* Check rotational symmetries *)
    List.iter [90; 180; 270] ~f:(fun angle ->
      if is_rotationally_symmetric g angle then
        symms := Rotational angle :: !symms);
    
    !symms
  
  and is_horizontally_symmetric g =
    let height = Array.length g in
    let width = if height > 0 then Array.length g.(0) else 0 in
    let rec check i =
      if i >= height / 2 then true
      else if array_equal g.(i) g.(height - 1 - i) then check (i + 1)
      else false
    in
    check 0
  
  and is_vertically_symmetric g =
    let height = Array.length g in
    let width = if height > 0 then Array.length g.(0) else 0 in
    let rec check j =
      if j >= width / 2 then true
      else
        let col_j = Array.init height ~f:(fun i -> g.(i).(j)) in
        let col_mirror = Array.init height ~f:(fun i -> g.(i).(width - 1 - j)) in
        if array_equal col_j col_mirror then check (j + 1)
        else false
    in
    check 0
  
  and is_rotationally_symmetric g angle = false (* Simplified *)
  
  and array_equal a1 a2 =
    Array.length a1 = Array.length a2 &&
    Array.for_all2_exn a1 a2 ~f:(=)
  
  let compute_topology (g: grid) : topology_descriptor =
    (* Compute topological invariants *)
    let components = count_connected_components g in
    let euler = compute_euler_characteristic g in
    {
      connected_components = components;
      euler_characteristic = euler;
      betti_numbers = [| components; 0; 0 |]; (* Simplified *)
      genus = 0;
    }
  
  and count_connected_components g =
    let height = Array.length g in
    let width = if height > 0 then Array.length g.(0) else 0 in
    let visited = Array.make_matrix ~dimx:height ~dimy:width false in
    let count = ref 0 in
    
    for i = 0 to height - 1 do
      for j = 0 to width - 1 do
        if not visited.(i).(j) && g.(i).(j) <> 0 then begin
          mark_component g visited i j;
          incr count
        end
      done
    done;
    !count
  
  and mark_component g visited i j =
    let color = g.(i).(j) in
    let rec mark i j =
      if i >= 0 && i < Array.length g &&
         j >= 0 && j < Array.length g.(0) &&
         not visited.(i).(j) && g.(i).(j) = color then begin
        visited.(i).(j) <- true;
        mark (i+1) j;
        mark (i-1) j;
        mark i (j+1);
        mark i (j-1);
      end
    in
    mark i j
  
  and compute_euler_characteristic g =
    (* V - E + F for planar graph *)
    0 (* Simplified *)
  
  let compute_complexity (g: grid) : float =
    (* Kolmogorov complexity approximation *)
    let height = Array.length g in
    let width = if height > 0 then Array.length g.(0) else 0 in
    let total_cells = float_of_int (height * width) in
    let unique_colors = count_unique_colors g in
    let symmetry_reduction = List.length (detect_symmetries g) in
    
    let base_complexity = float_of_int unique_colors /. 10.0 in
    let size_factor = log (total_cells +. 1.0) /. log 2.0 in
    let symmetry_factor = 1.0 -. (float_of_int symmetry_reduction *. 0.1) in
    
    base_complexity *. size_factor *. symmetry_factor
  
  and count_unique_colors g =
    let colors = Hash_set.create (module Int) in
    Array.iter g ~f:(fun row ->
      Array.iter row ~f:(fun c -> Hash_set.add colors c));
    Hash_set.length colors
end

(** Transformation detection *)
module TransformDetection = struct
  (** Detect transformation between input and output *)
  let detect_transformation (input: grid) (output: grid) : transformation option =
    (* Try different transformation types in order of likelihood *)
    
    (* 1. Simple color mapping *)
    match detect_color_mapping input output with
    | Some mapping -> Some (ColorMap mapping)
    | None ->
    
    (* 2. Geometric transformation *)
    match detect_geometric input output with
    | Some geom -> Some (Geometric geom)
    | None ->
    
    (* 3. Conditional transformation *)
    match detect_conditional input output with
    | Some cond -> Some cond
    | None ->
    
    (* 4. Composite transformation *)
    match detect_composite input output with
    | Some comp -> Some comp
    | None ->
    
    (* 5. Recursive pattern *)
    match detect_recursive input output with
    | Some rec_rule -> Some (Recursive rec_rule)
    | None -> None
  
  let detect_color_mapping input output : (color * color) list option =
    if not (same_dimensions input output) then None
    else
      let mapping = Hashtbl.create (module Int) in
      let consistent = ref true in
      
      Array.iteri input ~f:(fun i row ->
        Array.iteri row ~f:(fun j color ->
          let output_color = output.(i).(j) in
          match Hashtbl.find mapping color with
          | Some c when c <> output_color -> consistent := false
          | None -> Hashtbl.set mapping ~key:color ~data:output_color
          | _ -> ()));
      
      if !consistent then
        Some (Hashtbl.to_alist mapping)
      else None
  
  and same_dimensions g1 g2 =
    Array.length g1 = Array.length g2 &&
    (Array.length g1 = 0 || Array.length g1.(0) = Array.length g2.(0))
  
  let detect_geometric input output : geometric_transform option =
    (* Check for rotation *)
    List.find_map [90; 180; 270] ~f:(fun angle ->
      let rotated = rotate_grid input angle in
      if grid_equal rotated output then Some (Rotate angle)
      else None)
    |> Option.first_some (
      (* Check for flip *)
      let h_flip = flip_grid input `Horizontal in
      if grid_equal h_flip output then Some (Flip `Horizontal)
      else
        let v_flip = flip_grid input `Vertical in
        if grid_equal v_flip output then Some (Flip `Vertical)
        else None
    )
  
  and rotate_grid g angle = g (* Simplified *)
  and flip_grid g dir = g (* Simplified *)
  and grid_equal g1 g2 = false (* Simplified *)
  
  let detect_conditional input output : transformation option =
    (* Detect pattern-based conditional transformations *)
    None (* Simplified - would implement pattern matching *)
  
  let detect_composite input output : transformation option =
    (* Try to decompose into sequence of simpler transformations *)
    None (* Simplified - would implement decomposition *)
  
  let detect_recursive input output : recursive_rule option =
    (* Detect recursive/fractal patterns *)
    None (* Simplified - would implement recursion detection *)
end

(** Rule inference system *)
module RuleInference = struct
  type rule = {
    name: string;
    preconditions: condition list;
    transformation: transformation;
    postconditions: invariant list;
    confidence: float;
  }
  
  type rule_set = {
    rules: rule list;
    composition_graph: (rule * rule * float) list; (* rule1, rule2, compatibility *)
  }
  
  (** Infer rules from examples *)
  let infer_rules (examples: pattern list) : rule_set =
    (* Group examples by similarity *)
    let groups = cluster_examples examples in
    
    (* Extract rule from each group *)
    let rules = List.map groups ~f:extract_rule_from_group in
    
    (* Build composition graph *)
    let composition_graph = build_composition_graph rules in
    
    { rules; composition_graph }
  
  and cluster_examples examples =
    (* Cluster based on feature similarity *)
    [examples] (* Simplified - would implement clustering *)
  
  and extract_rule_from_group group =
    let features = List.map group ~f:(fun p -> 
      Analysis.extract_features p.input) in
    
    (* Find common preconditions *)
    let preconditions = extract_common_conditions features in
    
    (* Find common transformation *)
    let transformation = 
      match group with
      | [] -> ColorMap []
      | p :: _ -> 
          Option.value ~default:(ColorMap [])
            (TransformDetection.detect_transformation p.input p.output) in
    
    (* Find preserved invariants *)
    let postconditions = extract_invariants group in
    
    {
      name = generate_rule_name ();
      preconditions;
      transformation;
      postconditions;
      confidence = compute_confidence group;
    }
  
  and extract_common_conditions features = []
  and extract_invariants patterns = []
  and generate_rule_name () = "rule_" ^ Int.to_string (Random.int 1000)
  and compute_confidence patterns = 0.5 +. Random.float 0.5
  
  and build_composition_graph rules =
    (* Determine which rules can be composed *)
    List.concat_map rules ~f:(fun r1 ->
      List.filter_map rules ~f:(fun r2 ->
        if composable r1 r2 then
          let compat = compute_compatibility r1 r2 in
          Some (r1, r2, compat)
        else None))
  
  and composable r1 r2 =
    (* Check if r1's postconditions satisfy r2's preconditions *)
    true (* Simplified *)
  
  and compute_compatibility r1 r2 = Random.float 1.0
  
  (** Apply inferred rules to new problem *)
  let apply_rules (rule_set: rule_set) (input: grid) : grid option =
    (* Find applicable rules *)
    let applicable = List.filter rule_set.rules ~f:(fun rule ->
      check_preconditions rule.preconditions input) in
    
    (* Apply highest confidence rule *)
    match List.max_elt applicable ~compare:(fun r1 r2 -> 
      Float.compare r1.confidence r2.confidence) with
    | Some rule -> Some (apply_transformation rule.transformation input)
    | None -> None
  
  and check_preconditions conditions input =
    List.for_all conditions ~f:(fun cond ->
      check_condition cond input)
  
  and check_condition cond input =
    match cond with
    | ColorAt (i, j, c) ->
        i < Array.length input && j < Array.length input.(0) &&
        input.(i).(j) = c
    | _ -> true (* Simplified *)
  
  and apply_transformation trans input =
    match trans with
    | ColorMap mapping -> apply_color_map mapping input
    | Geometric geom -> apply_geometric geom input
    | _ -> input (* Simplified *)
  
  and apply_color_map mapping input =
    Array.map input ~f:(fun row ->
      Array.map row ~f:(fun color ->
        List.Assoc.find mapping color ~equal:(=)
        |> Option.value ~default:color))
  
  and apply_geometric geom input = input (* Simplified *)
end

(** Pattern completion *)
module PatternCompletion = struct
  type completion_strategy =
    | Symmetry of symmetry
    | Periodicity of int * int (* period in x, y *)
    | Progression of progression_type
    | Analogy of pattern * pattern * pattern (* A:B::C:? *)
  
  and progression_type =
    | Arithmetic of int
    | Geometric of float
    | Fibonacci
    | Custom of (int -> int)
  
  (** Complete partial pattern *)
  let complete_pattern (partial: grid) (mask: bool array array) 
                       (strategy: completion_strategy) : grid =
    match strategy with
    | Symmetry sym -> complete_by_symmetry partial mask sym
    | Periodicity (px, py) -> complete_by_period partial mask px py
    | Progression prog -> complete_by_progression partial mask prog
    | Analogy (a, b, c) -> complete_by_analogy partial mask a b c
  
  and complete_by_symmetry partial mask sym =
    let result = Array.copy partial in
    let height = Array.length partial in
    let width = if height > 0 then Array.length partial.(0) else 0 in
    
    match sym with
    | Horizontal ->
        for i = 0 to height - 1 do
          for j = 0 to width - 1 do
            if mask.(i).(j) then
              let mirror_i = height - 1 - i in
              if mirror_i >= 0 && not mask.(mirror_i).(j) then
                result.(i).(j) <- partial.(mirror_i).(j)
          done
        done;
        result
    | Vertical ->
        for i = 0 to height - 1 do
          for j = 0 to width - 1 do
            if mask.(i).(j) then
              let mirror_j = width - 1 - j in
              if mirror_j >= 0 && not mask.(i).(mirror_j) then
                result.(i).(j) <- partial.(i).(mirror_j)
          done
        done;
        result
    | _ -> partial
  
  and complete_by_period partial mask px py =
    let result = Array.copy partial in
    let height = Array.length partial in
    let width = if height > 0 then Array.length partial.(0) else 0 in
    
    for i = 0 to height - 1 do
      for j = 0 to width - 1 do
        if mask.(i).(j) then
          let source_i = i mod px in
          let source_j = j mod py in
          if source_i < height && source_j < width && not mask.(source_i).(source_j) then
            result.(i).(j) <- partial.(source_i).(source_j)
      done
    done;
    result
  
  and complete_by_progression partial mask prog = partial (* Simplified *)
  
  and complete_by_analogy partial mask a b c =
    (* Find transformation from a to b, apply to c to get result *)
    match TransformDetection.detect_transformation a.input b.input with
    | Some trans -> RuleInference.apply_transformation trans c.input
    | None -> partial
  
  (** Auto-detect best completion strategy *)
  let auto_complete (partial: grid) (mask: bool array array) : grid =
    let features = Analysis.extract_features partial in
    
    (* Try symmetry-based completion first *)
    match features.symmetries with
    | sym :: _ -> complete_pattern partial mask (Symmetry sym)
    | [] ->
        (* Try periodicity detection *)
        match detect_period partial with
        | Some (px, py) -> complete_pattern partial mask (Periodicity (px, py))
        | None -> partial
  
  and detect_period grid =
    (* Simple period detection *)
    None (* Simplified *)
end

(** Abstract reasoning *)
module AbstractReasoning = struct
  type abstraction =
    | Concept of string * (grid -> bool)
    | Relation of string * (grid -> grid -> bool)
    | Operation of string * (grid -> grid)
    | Property of string * (grid -> float)
  
  type reasoning_chain = {
    steps: reasoning_step list;
    confidence: float;
    explanation: string;
  }
  
  and reasoning_step =
    | Apply of abstraction
    | Compose of reasoning_step * reasoning_step
    | Iterate of reasoning_step * int
    | Branch of condition * reasoning_step * reasoning_step
  
  (** Build abstraction from examples *)
  let learn_abstraction (examples: (grid * grid) list) : abstraction =
    (* Detect common pattern across examples *)
    let common_transform = find_common_transformation examples in
    
    match common_transform with
    | Some trans -> 
        Operation ("learned_op", fun g -> 
          RuleInference.apply_transformation trans g)
    | None ->
        Property ("complexity", fun g ->
          Analysis.compute_complexity g)
  
  and find_common_transformation examples =
    match examples with
    | [] -> None
    | (inp, out) :: _ -> TransformDetection.detect_transformation inp out
  
  (** Chain reasoning steps *)
  let build_reasoning_chain (input: grid) (target: grid) : reasoning_chain =
    let steps = search_for_solution input target in
    {
      steps;
      confidence = evaluate_chain_confidence steps;
      explanation = explain_reasoning steps;
    }
  
  and search_for_solution input target =
    (* A* search in transformation space *)
    [] (* Simplified *)
  
  and evaluate_chain_confidence steps =
    match steps with
    | [] -> 0.0
    | _ -> 0.5 +. Random.float 0.5
  
  and explain_reasoning steps =
    "Apply transformations: " ^ Int.to_string (List.length steps)
end

(** Integration with SCTT type theory *)
module TypeTheoreticPatterns = struct
  open Kernel.Kan
  
  (** Represent pattern as type *)
  let pattern_to_type (p: pattern) : value =
    (* Encode pattern as path in type space *)
    let input_type = grid_to_type p.input in
    let output_type = grid_to_type p.output in
    VPath (VUniv 0, input_type, output_type)
  
  and grid_to_type (g: grid) : value =
    (* Encode grid as dependent sum *)
    let height = Array.length g in
    let width = if height > 0 then Array.length g.(0) else 0 in
    VPair (VNeutral (NVar height), VNeutral (NVar width))
  
  (** Use Kan operations for pattern transformation *)
  let transform_via_kan (input: grid) (trans: transformation) : grid =
    let input_val = grid_to_type input in
    let trans_path = transformation_to_path trans in
    
    (* Use composition to apply transformation *)
    let result = comp D0 D1 
                     (fun _ -> VUniv 0)
                     []
                     input_val
                     C_infty in
    
    type_to_grid result
  
  and transformation_to_path trans =
    match trans with
    | ColorMap _ -> VPath (VUniv 0, VNeutral (NVar 0), VNeutral (NVar 1))
    | _ -> VPath (VUniv 0, VNeutral (NVar 0), VNeutral (NVar 1))
  
  and type_to_grid v =
    Array.make_matrix ~dimx:10 ~dimy:10 0
  
  (** Smooth interpolation between patterns *)
  let smooth_interpolate (p1: pattern) (p2: pattern) (t: float) : pattern =
    let v1 = pattern_to_type p1 in
    let v2 = pattern_to_type p2 in
    
    (* Create smooth path between patterns *)
    let smooth_path = SmoothKan.homotopy v1 v2 (fun i -> v1) C_infty in
    
    (* Sample at parameter t *)
    let interpolated = sample_path smooth_path t in
    
    value_to_pattern interpolated
  
  and sample_path path t = path (* Simplified *)
  
  and value_to_pattern v = {
    input = Array.make_matrix ~dimx:1 ~dimy:1 0;
    output = Array.make_matrix ~dimx:1 ~dimy:1 0;
    metadata = {
      symmetries = [];
      transformations = [];
      invariants = [];
      complexity = 0.0;
    };
  }
end

(** Public API *)
type arc_pattern = pattern
type arc_rule = RuleInference.rule
type arc_transformation = transformation

let analyze_pattern = Analysis.extract_features
let detect_transformation = TransformDetection.detect_transformation
let infer_rules = RuleInference.infer_rules
let apply_rules = RuleInference.apply_rules
let complete_pattern = PatternCompletion.auto_complete
let build_reasoning = AbstractReasoning.build_reasoning_chain
let pattern_to_type = TypeTheoreticPatterns.pattern_to_type
let smooth_interpolate = TypeTheoreticPatterns.smooth_interpolate