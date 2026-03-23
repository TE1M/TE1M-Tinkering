import bcrypt

salt = bcrypt.gensalt(rounds=12)
password = bcrypt.hashpw(b'amogus', salt)
print(password)
