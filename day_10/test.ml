(* type opus_card = {
  available_tickets : unit -> int;
  buy_tickets : float -> float;
  ride: unit -> unit;
}

let rec div_by amt k =
  if amt <= 0. then
    (0, amt)
  else
    let (n, change) = div_by (amt -. k) k in
    (n + 1, change)

let new_card () =
  let tickets = ref 0 in
  {
    available_tickets = (fun () -> !tickets);
    buy_tickets = 
    let rec buy money change =
      if money < 3.5 then change else
        if money < 29.5 then 
          let (new_tickets, new_change) = div_by money 3.5 in

  } *)