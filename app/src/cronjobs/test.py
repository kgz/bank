# generate a list of aaa, aab
names = [chr(i) + chr(j) + chr(k) for i in range(97, 97+26) for j in range(97, 97+26) for k in range(97, 97+26)]

import os
import requests


def get_data(name):
    url = 'https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player={}'.format(name)
    r = requests.get(url)
    # if 200

    return r.status_code == 200


def main():
    possible_names = []
    for  index, name in enumerate(names):
        if not get_data(name):
            possible_names.append(name)

        os.system('cls' if os.name == 'nt' else 'clear')

        # print percent complete and possible names
        print('Percent complete: {}%'.format(round((index / len(names)) * 100, 2)))
        print('Possible names: {}'.format(possible_names))

    with open('possible_names.txt', 'w') as f:
        f.write('\n'.join(possible_names))

if __name__ == '__main__':
    main()

        
    


