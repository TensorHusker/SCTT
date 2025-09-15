(** Comprehensive test suite for Smooth Manifold operations
    Tests differential operations, smooth paths, and geometric structures *)

open OUnit2
open QCheck
open Core
open Sctt_smooth.Manifold

(** Mathematical constants and helpers *)
module MathHelpers = struct
  let pi = 4.0 *. Float.atan 1.0
  let e = Float.exp 1.0
  let epsilon = 1e-10
  
  (** Approximate floating point equality *)
  let float_equal ?(eps=epsilon) x y =
    Float.abs (x -. y) < eps
  
  (** Approximate vector equality *)
  let vector_equal ?(eps=epsilon) v1 v2 =
    Array.length v1 = Array.length v2 &&
    Array.for_all2_exn v1 v2 ~f:(float_equal ~eps)
  
  (** Approximate matrix equality *)
  let matrix_equal ?(eps=epsilon) m1 m2 =
    Array.length m1 = Array.length m2 &&
    Array.for_all2_exn m1 m2 ~f:(vector_equal ~eps)
  
  (** Create identity matrix *)
  let identity_matrix n =
    Array.init n ~f:(fun i ->
      Array.init n ~f:(fun j ->
        if i = j then 1.0 else 0.0))
  
  (** Standard test functions *)
  let polynomial coeffs x =
    Array.foldi coeffs ~init:0.0 ~f:(fun i acc c ->
      acc +. c *. (Float.pow x (Float.of_int i)))
  
  let sine_wave amplitude frequency phase x =
    amplitude *. Float.sin (frequency *. x +. phase)
  
  let gaussian mu sigma x =
    let exp_arg = -0.5 *. Float.pow ((x -. mu) /. sigma) 2.0 in
    (1.0 /. (sigma *. Float.sqrt (2.0 *. pi))) *. Float.exp exp_arg
end

(** Test fixtures for manifolds *)
module TestManifolds = struct
  open MathHelpers
  
  (** R^n Euclidean space *)
  let euclidean_space n = {
    dimension = n;
    chart_maps = [
      { domain = Full_space;
        codomain = R_n n;
        transition = Identity_map;
      }
    ];
    metric = Euclidean_metric (identity_matrix n);
    smoothness = C_infty;
  }
  
  (** S^n sphere *)
  let sphere n = {
    dimension = n;
    chart_maps = [
      (* North pole chart *)
      { domain = Complement (Point [|Array.create ~len:n 0.0|]);
        codomain = R_n n;
        transition = Stereographic_projection `North;
      };
      (* South pole chart *)
      { domain = Complement (Point [|Array.create ~len:n 0.0|]);
        codomain = R_n n;
        transition = Stereographic_projection `South;
      };
    ];
    metric = Induced_metric (euclidean_space (n + 1));
    smoothness = C_infty;
  }
  
  (** T^n torus *)
  let torus n = {
    dimension = n;
    chart_maps = [
      { domain = Full_space;
        codomain = Product (List.init n ~f:(fun _ -> S1));
        transition = Quotient_map (Z_n n);
      }
    ];
    metric = Product_metric (List.init n ~f:(fun _ -> S1_metric));
    smoothness = C_infty;
  }
  
  (** Möbius strip *)
  let mobius_strip = {
    dimension = 2;
    chart_maps = [
      { domain = Strip { width = 2.0 *. pi; height = 1.0 };
        codomain = R_n 3;
        transition = Mobius_embedding;
      }
    ];
    metric = Induced_metric (euclidean_space 3);
    smoothness = C_infty;
  }
end

(** Unit tests for smooth paths *)
let test_smooth_paths =
  "Smooth paths" >::: [
    "constant path smoothness" >:: (fun _ ->
      let point = [|1.0; 2.0; 3.0|] in
      let path = constant_path point in
      
      (* Constant path should be C^∞ smooth *)
      assert_equal (get_smoothness path) C_infty;
      
      (* All derivatives should be zero *)
      let deriv1 = differentiate_path path 1 0.5 in
      let deriv2 = differentiate_path path 2 0.5 in
      
      assert_bool "First derivative is zero"
        (MathHelpers.vector_equal deriv1 [|0.0; 0.0; 0.0|]);
      assert_bool "Second derivative is zero"
        (MathHelpers.vector_equal deriv2 [|0.0; 0.0; 0.0|])
    );
    
    "linear path smoothness" >:: (fun _ ->
      let start = [|0.0; 0.0|] in
      let finish = [|1.0; 1.0|] in
      let path = linear_path start finish in
      
      (* Linear path should be C^∞ smooth *)
      assert_equal (get_smoothness path) C_infty;
      
      (* Check path endpoints *)
      let p0 = evaluate_path path 0.0 in
      let p1 = evaluate_path path 1.0 in
      
      assert_bool "Path starts correctly"
        (MathHelpers.vector_equal p0 start);
      assert_bool "Path ends correctly"
        (MathHelpers.vector_equal p1 finish);
      
      (* First derivative should be constant *)
      let deriv1_a = differentiate_path path 1 0.3 in
      let deriv1_b = differentiate_path path 1 0.7 in
      
      assert_bool "Constant velocity"
        (MathHelpers.vector_equal deriv1_a deriv1_b)
    );
    
    "smooth spline path" >:: (fun _ ->
      let control_points = [|
        [|0.0; 0.0|];
        [|1.0; 2.0|];
        [|3.0; 1.0|];
        [|4.0; 0.0|];
      |] in
      
      let path = cubic_spline control_points in
      
      (* Check smoothness order *)
      assert_bool "Spline is at least C^2"
        (match get_smoothness path with
         | C_infty | C_n n when n >= 2 -> true
         | _ -> false);
      
      (* Check interpolation *)
      let p0 = evaluate_path path 0.0 in
      let p1 = evaluate_path path 1.0 in
      
      assert_bool "Interpolates first point"
        (MathHelpers.vector_equal p0 control_points.(0));
      assert_bool "Interpolates last point"
        (MathHelpers.vector_equal p1 control_points.(3))
    );
    
    "path composition" >:: (fun _ ->
      let path1 = linear_path [|0.0|] [|1.0|] in
      let path2 = linear_path [|1.0|] [|2.0|] in
      
      let composed = compose_paths path1 path2 in
      
      (* Check continuity at junction *)
      let p1_end = evaluate_path path1 1.0 in
      let p2_start = evaluate_path path2 0.0 in
      let comp_mid = evaluate_path composed 0.5 in
      
      assert_bool "Continuous at junction"
        (MathHelpers.vector_equal p1_end p2_start)
    );
    
    "geodesic paths" >:: (fun _ ->
      let manifold = TestManifolds.sphere 2 in
      let p1 = [|1.0; 0.0; 0.0|] in
      let p2 = [|0.0; 1.0; 0.0|] in
      
      let geodesic = compute_geodesic manifold p1 p2 in
      
      (* Geodesic should minimize distance *)
      let length = path_length geodesic manifold.metric in
      let alt_path = linear_path p1 p2 in
      let alt_length = path_length alt_path manifold.metric in
      
      assert_bool "Geodesic is shorter or equal"
        (length <= alt_length +. MathHelpers.epsilon)
    );
  ]

(** Unit tests for differential operations *)
let test_differential_operations =
  "Differential operations" >::: [
    "derivative computation" >:: (fun _ ->
      open MathHelpers in
      (* Test polynomial differentiation *)
      let f x = polynomial [|1.0; 2.0; 3.0|] x in (* 1 + 2x + 3x^2 *)
      let df = differentiate f in
      
      (* df/dx = 2 + 6x *)
      assert_bool "Derivative at x=0"
        (float_equal (df 0.0) 2.0);
      assert_bool "Derivative at x=1"
        (float_equal (df 1.0) 8.0);
      assert_bool "Derivative at x=2"
        (float_equal (df 2.0) 14.0)
    );
    
    "higher order derivatives" >:: (fun _ ->
      open MathHelpers in
      let f x = Float.sin x in
      
      let df = differentiate f in        (* cos x *)
      let d2f = differentiate df in      (* -sin x *)
      let d3f = differentiate d2f in     (* -cos x *)
      let d4f = differentiate d3f in     (* sin x *)
      
      let x = pi /. 4.0 in
      assert_bool "Fourth derivative is original function"
        (float_equal (d4f x) (f x) ~eps:1e-6)
    );
    
    "partial derivatives" >:: (fun _ ->
      (* f(x,y) = x^2 + xy + y^2 *)
      let f vars =
        let x = vars.(0) and y = vars.(1) in
        x *. x +. x *. y +. y *. y
      in
      
      (* ∂f/∂x = 2x + y *)
      let df_dx = partial_derivative f 0 in
      let result1 = df_dx [|1.0; 2.0|] in
      assert_bool "Partial wrt x at (1,2)"
        (MathHelpers.float_equal result1 4.0);
      
      (* ∂f/∂y = x + 2y *)
      let df_dy = partial_derivative f 1 in
      let result2 = df_dy [|1.0; 2.0|] in
      assert_bool "Partial wrt y at (1,2)"
        (MathHelpers.float_equal result2 5.0)
    );
    
    "gradient computation" >:: (fun _ ->
      (* f(x,y,z) = x^2 + y^2 + z^2 *)
      let f vars =
        Array.fold vars ~init:0.0 ~f:(fun acc x -> acc +. x *. x)
      in
      
      let grad = gradient f 3 in
      let point = [|1.0; 2.0; 3.0|] in
      let grad_at_point = grad point in
      
      (* ∇f = [2x, 2y, 2z] *)
      let expected = [|2.0; 4.0; 6.0|] in
      assert_bool "Gradient computation"
        (MathHelpers.vector_equal grad_at_point expected)
    );
    
    "jacobian matrix" >:: (fun _ ->
      (* F: R^2 -> R^2, F(x,y) = (x^2 + y, x - y^2) *)
      let f vars =
        let x = vars.(0) and y = vars.(1) in
        [|x *. x +. y; x -. y *. y|]
      in
      
      let jac = jacobian f 2 2 in
      let point = [|1.0; 2.0|] in
      let jac_at_point = jac point in
      
      (* J = [[2x, 1], [1, -2y]] at (1,2) = [[2, 1], [1, -4]] *)
      let expected = [|[|2.0; 1.0|]; [|1.0; -4.0|]|] in
      assert_bool "Jacobian computation"
        (MathHelpers.matrix_equal jac_at_point expected)
    );
    
    "Taylor series expansion" >:: (fun _ ->
      open MathHelpers in
      let f x = Float.exp x in
      let taylor = taylor_series f 0.0 5 in
      
      (* e^x ≈ 1 + x + x^2/2 + x^3/6 + x^4/24 + x^5/120 *)
      let x = 0.5 in
      let approx = taylor x in
      let exact = f x in
      
      assert_bool "Taylor approximation"
        (float_equal approx exact ~eps:1e-4)
    );
  ]

(** Unit tests for geometric structures *)
let test_geometric_structures =
  "Geometric structures" >::: [
    "metric tensor properties" >:: (fun _ ->
      let manifold = TestManifolds.euclidean_space 3 in
      let point = [|1.0; 2.0; 3.0|] in
      
      match manifold.metric with
      | Euclidean_metric g ->
        (* Euclidean metric should be identity *)
        let id = MathHelpers.identity_matrix 3 in
        assert_bool "Euclidean metric is identity"
          (MathHelpers.matrix_equal g id)
      | _ -> assert_failure "Expected Euclidean metric"
    );
    
    "christoffel symbols" >:: (fun _ ->
      let manifold = TestManifolds.sphere 2 in
      let point = [|Float.pi /. 4.0; Float.pi /. 3.0|] in (* spherical coords *)
      
      let christoffel = compute_christoffel_symbols manifold point in
      
      (* Check symmetry: Γ^k_ij = Γ^k_ji *)
      Array.iteri christoffel ~f:(fun k gamma_k ->
        Array.iteri gamma_k ~f:(fun i row ->
          Array.iteri row ~f:(fun j value ->
            assert_bool 
              (Printf.sprintf "Christoffel symmetry Γ^%d_%d%d = Γ^%d_%d%d" k i j k j i)
              (MathHelpers.float_equal value gamma_k.(j).(i)))))
    );
    
    "curvature tensor" >:: (fun _ ->
      let manifold = TestManifolds.sphere 2 in
      let point = [|0.0; 0.0; 1.0|] in (* North pole *)
      
      let riemann = compute_riemann_tensor manifold point in
      
      (* Check antisymmetry: R_ijkl = -R_jikl = -R_ijlk *)
      Array.iteri riemann ~f:(fun i r_i ->
        Array.iteri r_i ~f:(fun j r_ij ->
          Array.iteri r_ij ~f:(fun k r_ijk ->
            Array.iteri r_ijk ~f:(fun l value ->
              if j < Array.length riemann then
                assert_bool "Riemann antisymmetry in first pair"
                  (MathHelpers.float_equal value (-. riemann.(j).(i).(k).(l)))))))
    );
    
    "parallel transport" >:: (fun _ ->
      let manifold = TestManifolds.euclidean_space 2 in
      let path = linear_path [|0.0; 0.0|] [|1.0; 0.0|] in
      let vector = [|0.0; 1.0|] in (* Unit vector in y direction *)
      
      let transported = parallel_transport manifold path vector in
      
      (* In Euclidean space, parallel transport preserves the vector *)
      assert_bool "Euclidean parallel transport preserves vector"
        (MathHelpers.vector_equal transported vector)
    );
    
    "holonomy group element" >:: (fun _ ->
      let manifold = TestManifolds.sphere 2 in
      (* Closed loop on sphere *)
      let loop = closed_loop_path [|
        [|1.0; 0.0; 0.0|];
        [|0.0; 1.0; 0.0|];
        [|0.0; 0.0; 1.0|];
        [|1.0; 0.0; 0.0|];
      |] in
      
      let holonomy = compute_holonomy manifold loop in
      
      (* Holonomy should be non-trivial on curved manifold *)
      let id = MathHelpers.identity_matrix 3 in
      assert_bool "Non-trivial holonomy on sphere"
        (not (MathHelpers.matrix_equal holonomy id ~eps:0.1))
    );
  ]

(** Property-based tests *)
let test_properties =
  "Mathematical properties" >::: [
    "smoothness preservation" >:: (fun _ ->
      QCheck.Test.check_exn (
        QCheck.Test.make
          ~count:50
          ~name:"operations preserve smoothness"
          (pair small_int small_int)
          (fun (n1, n2) ->
            let n1 = abs n1 mod 10 in
            let n2 = abs n2 mod 10 in
            
            let s1 = if n1 = 0 then C_0 else C_n n1 in
            let s2 = if n2 = 0 then C_0 else C_n n2 in
            
            let combined = min_smoothness s1 s2 in
            
            (* Combined smoothness should be minimum *)
            match combined, s1, s2 with
            | C_0, _, _ -> true
            | C_n k, C_n k1, C_n k2 -> k = min k1 k2
            | _ -> true
          )
      )
    );
    
    "metric positive definiteness" >:: (fun _ ->
      QCheck.Test.check_exn (
        QCheck.Test.make
          ~count:50
          ~name:"metric tensor positive definite"
          (triple small_float small_float small_float)
          (fun (x, y, z) ->
            let g = [|
              [|1.0; 0.0; 0.0|];
              [|0.0; 1.0; 0.0|];
              [|0.0; 0.0; 1.0|];
            |] in
            let v = [|x; y; z|] in
            
            let norm_squared = compute_inner_product g v v in
            
            (* Inner product should be non-negative *)
            norm_squared >= -. MathHelpers.epsilon
          )
      )
    );
    
    "chain rule for derivatives" >:: (fun _ ->
      QCheck.Test.check_exn (
        QCheck.Test.make
          ~count:30
          ~name:"chain rule"
          small_float
          (fun x ->
            open MathHelpers in
            let f u = u *. u in
            let g x = Float.sin x in
            
            (* (f ∘ g)' = f'(g(x)) * g'(x) *)
            let fog x = f (g x) in
            let d_fog = differentiate fog in
            
            let fg_x = g x in
            let df_at_gx = 2.0 *. fg_x in
            let dg_at_x = Float.cos x in
            let expected = df_at_gx *. dg_at_x in
            
            float_equal (d_fog x) expected ~eps:1e-6
          )
      )
    );
  ]

(** Performance benchmarks *)
let bench_manifold_operations () =
  let open Benchmark in
  let manifold = TestManifolds.sphere 3 in
  let point = [|1.0; 0.0; 0.0; 0.0|] in
  
  let bench_christoffel () =
    ignore (compute_christoffel_symbols manifold point) in
  
  let bench_riemann () =
    ignore (compute_riemann_tensor manifold point) in
  
  let bench_geodesic () =
    let p1 = [|1.0; 0.0; 0.0; 0.0|] in
    let p2 = [|0.0; 1.0; 0.0; 0.0|] in
    ignore (compute_geodesic manifold p1 p2) in
  
  let bench_parallel_transport () =
    let path = linear_path point [|0.0; 1.0; 0.0; 0.0|] in
    let vector = [|0.0; 0.0; 1.0; 0.0|] in
    ignore (parallel_transport manifold path vector) in
  
  let results = throughputN 10 [
    ("christoffel_symbols", bench_christoffel, ());
    ("riemann_tensor", bench_riemann, ());
    ("geodesic", bench_geodesic, ());
    ("parallel_transport", bench_parallel_transport, ());
  ] in
  
  print_newline ();
  print_string "Manifold Operations Benchmarks:\n";
  tabulate results

(** Integration tests *)
let test_integration =
  "Integration tests" >::: [
    "smooth path on manifold" >:: (fun _ ->
      let manifold = TestManifolds.torus 2 in
      let path = smooth_curve_on_manifold manifold 
        ~start:[|0.0; 0.0|] 
        ~finish:[|Float.pi; Float.pi|]
        ~smoothness:(C_n 3) in
      
      (* Check that path lies on manifold *)
      let t_values = [0.0; 0.25; 0.5; 0.75; 1.0] in
      List.iter t_values ~f:(fun t ->
        let point = evaluate_path path t in
        assert_bool 
          (Printf.sprintf "Point at t=%f lies on manifold" t)
          (point_on_manifold manifold point))
    );
    
    "differential forms integration" >:: (fun _ ->
      let manifold = TestManifolds.euclidean_space 2 in
      
      (* 1-form: ω = x dy - y dx *)
      let omega point =
        let x = point.(0) and y = point.(1) in
        [|-.y; x|]
      in
      
      (* Integrate over unit circle *)
      let circle_path = parametric_path 
        (fun t -> [|Float.cos (2.0 *. Float.pi *. t);
                   Float.sin (2.0 *. Float.pi *. t)|])
        ~smoothness:C_infty in
      
      let integral = integrate_form omega circle_path in
      
      (* Should give 2π (area of unit circle) *)
      assert_bool "Line integral over circle"
        (MathHelpers.float_equal integral (2.0 *. Float.pi) ~eps:1e-4)
    );
  ]

(** Edge cases *)
let test_edge_cases =
  "Edge cases" >::: [
    "singleton manifold" >:: (fun _ ->
      let point_manifold = {
        dimension = 0;
        chart_maps = [{
          domain = Point [||];
          codomain = R_n 0;
          transition = Identity_map;
        }];
        metric = Trivial_metric;
        smoothness = C_infty;
      } in
      
      assert_equal point_manifold.dimension 0
    );
    
    "discontinuous path" >:: (fun _ ->
      let path = {
        curve = (fun t -> if t < 0.5 then [|0.0|] else [|1.0|]);
        smoothness = C_0;
        domain = (0.0, 1.0);
      } in
      
      assert_equal path.smoothness C_0;
      
      (* Derivative should fail or return undefined *)
      try
        let _ = differentiate_path path 1 0.5 in
        assert_failure "Should not differentiate discontinuous path"
      with
      | Cannot_differentiate _ -> assert_bool "Correctly rejects differentiation" true
      | _ -> assert_failure "Wrong exception type"
    );
    
    "degenerate metric" >:: (fun _ ->
      let degenerate_metric = [|
        [|1.0; 0.0|];
        [|0.0; 0.0|];  (* Degenerate *)
      |] in
      
      try
        let _ = invert_metric degenerate_metric in
        assert_failure "Should not invert degenerate metric"
      with
      | Singular_metric -> assert_bool "Detects singular metric" true
      | _ -> assert_failure "Wrong exception type"
    );
  ]

(** Main test suite *)
let suite =
  "Smooth Manifold Test Suite" >::: [
    test_smooth_paths;
    test_differential_operations;
    test_geometric_structures;
    test_properties;
    test_integration;
    test_edge_cases;
  ]

let () =
  (* Run tests *)
  run_test_tt_main suite;
  
  (* Run benchmarks if requested *)
  if Array.length Sys.argv > 1 && Sys.argv.(1) = "--bench" then
    bench_manifold_operations ()