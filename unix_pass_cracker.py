import bcrypt

def main():
    passfile = open("passwords.txt", 'r')
    for l in passfile.readlines():
        if ':' in l:
            # Split first argument(before ':')
            user = l.split(':')[0]
            crypt_pass = l.split(':')[1].strip().encode('utf-8')
            print(f"[+] Cracking password for: " + user)
            testpass(crypt_pass)

def testpass(crypt_pass):
    dictfile = open("dictionary.txt", 'r')
    for w in dictfile.readlines():
        w = w.strip('\n').encode('utf-8')
        # You can pass either a salt or the previously hashed password (containing the salt). This allows for proper comparison.
        if bcrypt.hashpw(w, crypt_pass) == crypt_pass:
            # needs to be decoded back from utf-8
            w = w.decode('utf-8')
            print(f'[+] Found password:' + w + '\n')
            return
    print("[+] No Password Found")
    return
        

if __name__ == "__main__":
    main()