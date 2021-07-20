
let rec squeeze n =
  if n == 1 then
    print_endline "NO"
  else
    if n mod 2 == 0 then
      squeeze (n / 2)
    else
      print_endline "YES";;

let cases = read_int () in


for i = 1 to cases do
  let n = read_int () in
  squeeze n
done