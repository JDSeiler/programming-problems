module IntSet = Set.Make(Int64)

(* 
Represent the possible pairings as a Bipartite Graph, with Boys on one side and Girls on the other.
For each edge (u,v), you can calculate the number of valid pairings by knowing the degrees of
u and v in the graph.

Procedure:
Sum of: for each edge u,v:
  b_degree = number of edges incident with u - 1
  g_degree = number of edges incident with v - 1
  We subtract 1 because one edge is shared (the edge being considered)

  all_other_edges = k-1
  number of possible pairs with u,v = all_other_edges - (b_degree+g_degree)

  return number of possible pairs
  
The answer is the above sum divided by 2. We divide by two because the above
algorithm considers the pairings a,b and b,a unique. But the problem does not.
So, by dividing by 2 we remove the "symmetrical"/"mirror" pairings. 
*)

(* Record for easily getting a,b, and k *)
type constraints = {
  a: int;
  b: int;
  k: int;
}

(* Helper for reading a,b, and k *)
let read_three_ints () =
  String.split_on_char ' ' (read_line ())
  |> List.map int_of_string
  |> fun list -> {
    a = List.nth list 0; 
    b = List.nth list 1; 
    k = List.nth list 2;
  }

(* So thankful there are lots of Haskell answers on stack overflow *)
(* Filters a list down to its unique elements *)
let rec uniq l =
  match l with
  | [] -> []
  | h :: tl -> h :: uniq (List.filter (fun e -> e != h) tl);;

(* Creates a simple array of pairs where each k is paired with how many times it appears *)
let init_assoc keys =
  let create_tuple k =
    (k, 0)
  in List.map create_tuple keys 
  |> Array.of_list

(* Increments the 2nd element of the pair at index k-1  *)
let update k assoc =
  let new_value = 
    match Array.get assoc k with
    | (k, v) -> (k, v+1)
  in Array.set assoc k new_value;;

(* 
The dream:
l => [1; 1; 1; 2; 3; 3;]
returns = [(1, 3); (2, 1); (3, 2)]
*)
let compute_vertex_degree l =
  let assoc = init_assoc (uniq l) in
  let increment k =
    update (k-1) assoc
  in List.iter increment l;
  assoc;;

let read_int_list () =
  String.split_on_char ' ' (read_line ()) 
  |> List.map int_of_string

let iter f n =
  for i = 1 to n do
    f n
  done;;

let num_cases = read_int () in
let process_test test_number = 
  (* let constraints = read_three_ints () in
  let bs = read_int_list in
  let gs = read_int_list in *)
  (* Almost there, just have to put all the bits together *)
  (* Need to use existing functions to compute vertex degree *)
  (* I think I need a lookup function so I can match the lists B and G *)
  ()
in iter process_test num_cases
