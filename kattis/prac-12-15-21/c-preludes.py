# Learned a new technique from: https://stackoverflow.com/a/36237166
# Read lines until EOF is reached
from sys import stdin

enharmonic_keys = [
    ("A#", "Bb"),
    ("C#", "Db"),
    ("D#", "Eb"),
    ("F#", "Gb"),
    ("G#", "Ab")
]

ctr = 1
for line in stdin:
    # t for test
    t_key, t_mode = line.split()

    result = "UNIQUE"
    # e for 'enharmonic'
    for e_key_sharp, e_key_flat in enharmonic_keys:
        if t_key == e_key_sharp:
            result = f"{e_key_flat} {t_mode}"
            break
        elif t_key == e_key_flat:
            result = f"{e_key_sharp} {t_mode}"
            break
        else:
            continue
    
    print(f"Case {ctr}: {result}")
    ctr += 1
