let rec print_list l =
  match l with
  | [] -> ()
  | [a] -> print_int a;
  | h :: tl -> print_int h; print_string " "; print_list tl;;

let rec make_list n =
  match n with
  | 0 -> []
  | x -> make_list (n-1) @ (x :: [])

let left_shift l =
  match l with
  | [] -> []
  | [a] -> [a]
  | h :: tl -> tl @ (h :: []);;

let cases = read_int () in
for i = 1 to cases do
  let n = read_int () in
  let l = make_list n in
  print_list (left_shift l); print_newline ();
done
