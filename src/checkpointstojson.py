#i felt a bit quirky so i used pickle to pickilfy stuff but pickle is shit and i need a real file 

import pickle
import json

data = []

with open('data.pkl', 'rb') as f:
    data = pickle.load(f)


newdata = []

for tup in data:
    newtup = []
    for num in tup:
        newtup.append(float(num))

    newdata.append(newtup)



json_object = json.dumps(newdata, indent=4)

 
# Writing to sample.json
with open("checkpoints.json", "w") as outfile:
    outfile.write(json_object)
