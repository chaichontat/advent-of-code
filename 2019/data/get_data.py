#%%
from pathlib import Path

import requests
import time

year = 2019
# day = 17
for day in range(17, 26):
    uri = f"https://adventofcode.com/{year}/day/{day}/input"
    response = requests.get(
        uri,
        cookies={
            "session": "53616c7465645f5f6a81946055a4190e20aaec56f0f962a1e0952a5d2002c087b0037ed39fe949b5d44d23925868620c",
        },
    )
    if response.status_code != 200:
        raise requests.ConnectionError
    
    Path(f"day{day:2d}.txt").write_text(response.text)
    print(f"Done Day {day}")
    time.sleep(0.1)

        

# %%
