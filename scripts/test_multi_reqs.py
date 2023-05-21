import dotenv
import requests
import os
import threading

def test_multi_requests():
    dotenv.load_dotenv()

    threads = []
    for i in range(5):
        data = {
            "class": "Mage",
            "uuid": "andrew_uuid",
            "name": "andrew"+str(i),
            "tower": {
                "owner": "Test Guild",
                "attackSpeed": 4,
                "damage": 4,
                "defense": 4,
                "health": 4,
                "territory": "Test Territory"
            }
        }

        def f():
            a = requests.post(f"http://{os.environ['CATHODE_HOST']}/submit_war_attempt", json=data)
            print(a.text, a.json())
        
        tr = threading.Thread(target=f)
        threads.append(tr)
        # this should only give one insertion to war_tower but 5 insertions in war_record
    
    for i in range(len(threads)):
        print("starting thread", i)
        threads[i].start()

    for x in threads:
        x.join()

test_multi_requests()