import struct
import mmap
import time
import numpy as np
import pickle

#memfile_path = "/home/adi/adi_programmer/mc-coords-mod/run/text.txt"
memfile_path = "/home/adi/.minecraft/text.txt"

checkpoints = []


try:
    with open(memfile_path, "r+b") as f:
        while True:
            time.sleep(.05)
            mm = mmap.mmap(f.fileno(), 16)
            line = mm.readline()
            if len(line) != 16:
                continue

            data = np.frombuffer(line, dtype=np.dtype('>f'))
            checkpoints.append((data[0], data[1], data[2]))
            print(checkpoints)

except KeyboardInterrupt:
    with open('data.pkl', 'wb') as f:  # Python 3: open(..., 'wb')
        pickle.dump(checkpoints, f)
        print("saved that shiiii")

