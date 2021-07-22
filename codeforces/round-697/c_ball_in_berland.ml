let print_list a =
  let print_elem e =
    print_int e;
    print_string " "
  in List.iter print_elem a;
  print_newline ();;

let print_tuple_list a =
  let print_elem e =
    let a,b = e in
    Printf.printf "(%d, %d) " a b
  in List.iter print_elem a;
  print_newline ();;

module MyInt = struct
  type t = int
  let compare a b = 
    if a < b then
      -1
    else if a > b then
      1
    else
      0
end

module IntSet = Set.Make(Int)

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

(* Ripped straight out of the Stdlib because this is only available since 4.05 *)
(* But codeforces is using 4.02.... *)
(* You -can- use a method from Str to split with a regex, but that could be slow. *)
let split_on_char sep s =
   let r = ref [] in
   let j = ref (String.length s) in
   for i = String.length s - 1 downto 0 do
     if String.unsafe_get s i = sep then begin
       r := String.sub s (i + 1) (!j - i - 1) :: !r;
       j := i
     end
   done;
   String.sub s 0 !j :: !r

(* Helper for reading a,b, and k *)
let read_three_ints () =
  split_on_char ' ' (read_line ())
  |> List.map int_of_string
  |> fun list -> {
    a = List.nth list 0; 
    b = List.nth list 1; 
    k = List.nth list 2;
  }

(* So thankful there are lots of Haskell answers on stack overflow *)
(* Filters a list down to its unique elements *)
(* This could be really slow.... *)
(* let rec uniq l =
  match l with
  | [] -> []
  | h :: tl -> h :: uniq (List.filter (fun e -> e != h) tl);; *)

(* Lets use a set instead *)
let uniq l =
  IntSet.of_list l |> IntSet.elements

(* Creates a simple array of pairs where each k is paired with how many times it appears *)
let init_assoc keys size =
  let assoc = Array.make size (0,0) in
  let create_tuple k =
    Array.set assoc (k-1) (k, 0)
  in List.iter create_tuple keys;
  assoc;;

(* Increments the 2nd element of the pair at index k-1  *)
let update k assoc =
  let new_value = 
    match Array.get assoc (k-1) with
    | (k, v) -> (k, v+1)
  in Array.set assoc (k-1) new_value;;

(* 
The dream:
l => [1; 1; 1; 2; 3; 3;]
returns = [(1, 3); (2, 1); (3, 2)]
*)
let compute_vertex_degree l size =
  let keys = l in
  let assoc = init_assoc keys size in
  let increment k =
    update k assoc
  in List.iter increment keys;
  assoc;;

let read_int_list () =
  split_on_char ' ' (read_line ())
  |> List.map int_of_string

let iter f n =
  for i = 1 to n do
    f n
  done;;

let fetch_degree degree_array node =
  let _, degree = degree_array.(node-1) in
  degree

let sum list =
  List.fold_left ( + ) 0 list

let halve n =
  n / 2

let lookup_edge n edges =
  Array.get edges (n-1)

let count_pairings edges b_degree g_degree k =
  let count_for_edge e =
    let b, g = e in
    let b_incident = (fetch_degree b_degree b) - 1 in
    let g_incident = (fetch_degree g_degree g) - 1 in
    (k-1) - (b_incident + g_incident)
  in List.map count_for_edge edges |> sum |> halve;;


let num_cases = read_int () in
let process_test test_number = 
  (* Read in all the stuff *)
  let constraints = read_three_ints () in
  let bs = read_int_list () in
  let gs = read_int_list () in
  (* Create the edges for looking up edge degrees *)
  let edges = List.combine bs gs in
  (* Compute the degrees *)
  let b_degree = compute_vertex_degree bs constraints.a in
  let g_degree = compute_vertex_degree gs constraints.b in
  (* Compute the number *)
  Printf.printf "%d\n" (count_pairings edges b_degree g_degree constraints.k)
in iter process_test num_cases
