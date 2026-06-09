# andor-rc-vouch-wrong: `useradd` is a NON-conforming establish (rc!=0 when the user
# already exists). Engine WRONGLY marks the converged useradd Replace (disposition pin:
# observable_matrix::xfail_nonconforming_...). The LINE-GRANULAR render masks it here
# (the || line has a Run leaf -> stays verbatim), so the apply is accidentally SAFE.
useradd deploy || mkdir /srv/app
