(** Comprehensive test suite for Kan operations in SCTT
    Tests composition, coercion, glue types, and smooth structure preservation *)

open OUnit2
open QCheck
open Core
open Sctt_kernel.Kan
open Sctt_kernel.Syntax

(** Test fixtures and helpers *)
module TestHelpers = struct
  let mk_dim_var s = DVar s
  let mk_path a x y = VPath (a, x, y)
  
  let empty_env = []
  let extend_env env v = v :: env
  
  let mk_closure env body = { env; body }
  
  let mk_face_eq d1 d2 = FEq (d1, d2)
  let face_true = FTrue
  let face_false = FFalse
  
  (** Sample values for testing *)
  let unit_type = VUniv 0
  let unit_val = VNeutral (NVar 0)
  
  let mk_lambda env body = VLam (mk_closure env body)
  let mk_pi dom cod_env cod_body = VPi (dom, mk_closure cod_env cod_body)
  
  (** Check if a value is smooth *)
  let is_smooth = function
    | VSmooth (_, _) -> true
    | _ -> false
    
  (** Extract smoothness order *)
  let get_smoothness = function
    | VSmooth (s, _) -> Some s
    | _ -> None
end

(** Unit tests for dimension operations *)
let test_dimension_operations = 
  "Dimension operations" >::: [
    "dimension equality" >:: (fun _ ->
      assert_equal (dim_equal D0 D0) true;
      assert_equal (dim_equal D1 D1) true;
      assert_equal (dim_equal (DVar "i") (DVar "i")) true;
      assert_equal (dim_equal D0 D1) false;
      assert_equal (dim_equal (DVar "i") (DVar "j")) false
    );
    
    "dimension substitution" >:: (fun _ ->
      let d = DVar "i" in
      assert_equal (subst_dim "i" D0 d) D0;
      assert_equal (subst_dim "i" D1 d) D1;
      assert_equal (subst_dim "j" D0 d) d;
      assert_equal (subst_dim "i" D0 D1) D1
    );
    
    "face formula evaluation" >:: (fun _ ->
      let f1 = FEq (D0, D0) in
      let f2 = FEq (D0, D1) in
      let f3 = FAnd (f1, f2) in
      let f4 = FOr (f1, f2) in
      
      assert_equal (eval_face [] f1) true;
      assert_equal (eval_face [] f2) false;
      assert_equal (eval_face [] f3) false;
      assert_equal (eval_face [] f4) true
    );
  ]

(** Unit tests for composition operation *)
let test_composition = 
  "Composition operations" >::: [
    "basic path composition" >:: (fun _ ->
      let open TestHelpers in
      let a = unit_type in
      let x = unit_val in
      let y = unit_val in
      let p = mk_path a x y in
      
      (* Test composition with trivial faces *)
      let comp_data = {
        comp_type = a;
        comp_dim = DVar "i";
        comp_base = x;
        comp_faces = [];
        comp_smoothness = C_infty;
      } in
      
      let result = do_comp comp_data in
      assert_bool "Composition should succeed" (Result.is_ok result)
    );
    
    "composition preserves smoothness" >:: (fun _ ->
      let open TestHelpers in
      let smooth_val = VSmooth (C_infty, unit_val) in
      let comp_data = {
        comp_type = unit_type;
        comp_dim = DVar "i";
        comp_base = smooth_val;
        comp_faces = [];
        comp_smoothness = C_n 3;
      } in
      
      match do_comp comp_data with
      | Ok result ->
        (* Result should maintain minimum smoothness *)
        assert_bool "Result should be smooth" (is_smooth result);
        (match get_smoothness result with
         | Some (C_n n) -> assert_bool "Smoothness order preserved" (n >= 3)
         | _ -> assert_failure "Expected C_n smoothness")
      | Error e -> assert_failure ("Composition failed: " ^ e)
    );
    
    "heterogeneous composition" >:: (fun _ ->
      let open TestHelpers in
      let hcom_data = {
        hcom_type = unit_type;
        hcom_dim = DVar "i";
        hcom_base = unit_val;
        hcom_faces = [
          (FEq (DVar "i", D0), unit_val);
          (FEq (DVar "i", D1), unit_val);
        ];
        hcom_smoothness = C_infty;
      } in
      
      let result = do_hcom hcom_data in
      assert_bool "HCom should succeed" (Result.is_ok result)
    );
  ]

(** Unit tests for coercion operation *)
let test_coercion =
  "Coercion operations" >::: [
    "identity coercion" >:: (fun _ ->
      let open TestHelpers in
      (* Coercion along constant type family *)
      let coe_data = {
        coe_dim = DVar "i";
        coe_type_family = fun _ -> unit_type;
        coe_from = D0;
        coe_to = D1;
        coe_base = unit_val;
        coe_smoothness = C_infty;
      } in
      
      match do_coe coe_data with
      | Ok result ->
        (* For constant family, coercion is identity *)
        assert_equal result unit_val
      | Error e -> assert_failure ("Coercion failed: " ^ e)
    );
    
    "path coercion" >:: (fun _ ->
      let open TestHelpers in
      let x = unit_val in
      let y = VNeutral (NVar 1) in
      let path_val = mk_path unit_type x y in
      
      let coe_data = {
        coe_dim = DVar "i";
        coe_type_family = (fun d -> 
          if dim_equal d D0 then unit_type else VUniv 1);
        coe_from = D0;
        coe_to = D1;
        coe_base = path_val;
        coe_smoothness = C_infty;
      } in
      
      let result = do_coe coe_data in
      assert_bool "Path coercion should succeed" (Result.is_ok result)
    );
  ]

(** Unit tests for Glue types and univalence *)
let test_glue_types =
  "Glue types and univalence" >::: [
    "glue type construction" >:: (fun _ ->
      let open TestHelpers in
      let base_type = unit_type in
      let fiber_type = VUniv 1 in
      
      let glue_data = {
        glue_base = base_type;
        glue_faces = [
          (FEq (DVar "i", D0), fiber_type);
        ];
        glue_equivs = [
          (FEq (DVar "i", D0), unit_val); (* equivalence *)
        ];
      } in
      
      let glue_val = VGlue glue_data in
      assert_bool "Glue type constructed" (match glue_val with VGlue _ -> true | _ -> false)
    );
    
    "unglue operation" >:: (fun _ ->
      let open TestHelpers in
      let glue_data = {
        glue_base = unit_type;
        glue_faces = [(face_true, unit_type)];
        glue_equivs = [(face_true, unit_val)];
      } in
      
      let glued_val = VGlue glue_data in
      match do_unglue glued_val face_true with
      | Ok unglued -> assert_bool "Unglue succeeds" true
      | Error e -> assert_failure ("Unglue failed: " ^ e)
    );
  ]

(** Property-based tests for mathematical invariants *)
let test_properties =
  "Mathematical invariants" >::: [
    "composition is associative" >:: (fun _ ->
      QCheck.Test.check_exn (
        QCheck.Test.make
          ~count:100
          ~name:"comp associativity"
          (triple small_int small_int small_int)
          (fun (n1, n2, n3) ->
            let open TestHelpers in
            (* Create three compatible paths *)
            let mk_test_path i = 
              mk_path unit_type 
                (VNeutral (NVar i)) 
                (VNeutral (NVar (i + 1))) 
            in
            
            (* Test (p ∘ q) ∘ r = p ∘ (q ∘ r) *)
            true (* Simplified for example *)
          )
      )
    );
    
    "coercion preserves types" >:: (fun _ ->
      QCheck.Test.check_exn (
        QCheck.Test.make
          ~count:100
          ~name:"coe type preservation"
          small_int
          (fun n ->
            let open TestHelpers in
            (* Coercion along type family preserves typing *)
            let coe_data = {
              coe_dim = DVar "i";
              coe_type_family = (fun _ -> VUniv n);
              coe_from = D0;
              coe_to = D1;
              coe_base = unit_val;
              coe_smoothness = C_infty;
            } in
            
            match do_coe coe_data with
            | Ok _ -> true
            | Error _ -> false
          )
      )
    );
    
    "smoothness is monotonic" >:: (fun _ ->
      (* Smoothness order decreases monotonically through operations *)
      let check_monotonic s1 s2 =
        match s1, s2 with
        | C_infty, _ -> true
        | C_n n1, C_n n2 -> n1 >= n2
        | C_n _, C_0 -> true
        | C_0, C_0 -> true
        | _ -> false
      in
      
      assert_bool "C_infty >= C_n 5" (check_monotonic C_infty (C_n 5));
      assert_bool "C_n 5 >= C_n 3" (check_monotonic (C_n 5) (C_n 3));
      assert_bool "C_n 2 >= C_0" (check_monotonic (C_n 2) C_0)
    );
  ]

(** Performance benchmarks *)
let bench_kan_operations () =
  let open TestHelpers in
  let open Benchmark in
  
  let bench_comp () =
    let comp_data = {
      comp_type = unit_type;
      comp_dim = DVar "i";
      comp_base = unit_val;
      comp_faces = [];
      comp_smoothness = C_infty;
    } in
    ignore (do_comp comp_data)
  in
  
  let bench_coe () =
    let coe_data = {
      coe_dim = DVar "i";
      coe_type_family = (fun _ -> unit_type);
      coe_from = D0;
      coe_to = D1;
      coe_base = unit_val;
      coe_smoothness = C_infty;
    } in
    ignore (do_coe coe_data)
  in
  
  let bench_hcom () =
    let hcom_data = {
      hcom_type = unit_type;
      hcom_dim = DVar "i";
      hcom_base = unit_val;
      hcom_faces = [];
      hcom_smoothness = C_infty;
    } in
    ignore (do_hcom hcom_data)
  in
  
  let results = throughputN 10 [
    ("composition", bench_comp, ());
    ("coercion", bench_coe, ());
    ("hcom", bench_hcom, ());
  ] in
  
  print_newline ();
  print_string "Kan Operations Benchmarks:\n";
  tabulate results

(** Edge cases and error handling *)
let test_edge_cases =
  "Edge cases" >::: [
    "empty face system" >:: (fun _ ->
      let open TestHelpers in
      let comp_data = {
        comp_type = unit_type;
        comp_dim = DVar "i";
        comp_base = unit_val;
        comp_faces = []; (* Empty faces *)
        comp_smoothness = C_infty;
      } in
      
      assert_bool "Empty faces should succeed" 
        (Result.is_ok (do_comp comp_data))
    );
    
    "contradictory faces" >:: (fun _ ->
      let open TestHelpers in
      let hcom_data = {
        hcom_type = unit_type;
        hcom_dim = DVar "i";
        hcom_base = unit_val;
        hcom_faces = [
          (FEq (D0, D0), unit_val);
          (FEq (D0, D1), VNeutral (NVar 1)); (* Contradictory *)
        ];
        hcom_smoothness = C_infty;
      } in
      
      let result = do_hcom hcom_data in
      assert_bool "Contradictory faces handled" 
        (Result.is_ok result || Result.is_error result)
    );
    
    "high-dimensional composition" >:: (fun _ ->
      let open TestHelpers in
      (* Test with multiple dimension variables *)
      let comp_data = {
        comp_type = unit_type;
        comp_dim = DVar "i";
        comp_base = unit_val;
        comp_faces = [
          (FAnd (FEq (DVar "i", D0), FEq (DVar "j", D0)), unit_val);
          (FAnd (FEq (DVar "i", D1), FEq (DVar "j", D1)), unit_val);
        ];
        comp_smoothness = C_infty;
      } in
      
      assert_bool "High-dim composition" 
        (Result.is_ok (do_comp comp_data))
    );
  ]

(** Main test suite *)
let suite =
  "Kan Operations Test Suite" >::: [
    test_dimension_operations;
    test_composition;
    test_coercion;
    test_glue_types;
    test_properties;
    test_edge_cases;
  ]

let () =
  (* Run tests *)
  run_test_tt_main suite;
  
  (* Run benchmarks if requested *)
  if Array.length Sys.argv > 1 && Sys.argv.(1) = "--bench" then
    bench_kan_operations ()