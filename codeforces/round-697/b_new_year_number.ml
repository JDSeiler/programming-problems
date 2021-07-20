let calculate_table () =
  let table = Hashtbl.create 1000000 in
  for a = 0 to 500 do
    for b = 0 to 500 do
      let new_value = (a*2020 + b*2021) in
      Hashtbl.add table new_value (a, b)
    done
  done;
  table;;

(* 
Have to implement this manually since it was introduced 
in OCaml 4.05 and Codeforces uses OCaml 4.02.
*)
let find_opt tbl n =
  try
    Some (Hashtbl.find tbl n)
  with Not_found -> None

let solution_exists n solutions =
  let maybe_sol = find_opt solutions n in
  match maybe_sol with
  | Some _ -> true
  | None -> false;;

let cases = read_int () in
let solutions = calculate_table () in
for i = 1 to cases do
  let test = read_int () in
  if solution_exists test solutions then
    print_endline "YES"
  else
    print_endline "NO"
done