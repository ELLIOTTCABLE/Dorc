module claims
open species
open words
one sig alice__frob_wobbles extends Wobble {} { speaker = alice of = frob }
one sig bob__spin_wobbles extends Wobble {} { speaker = bob of = spin }
one sig bob__twirl_wobbles_on_the_sprocket_heap extends Wobble {} { speaker = bob of = sprocket_heap }
fun alice_kit: set Claim { alice__frob_wobbles + bob__spin_wobbles }
