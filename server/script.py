import sys
import sqlite3

def parse_query_string(query: str) -> dict:
    result = {}
    for pair in query.split("&"):
        if "=" in pair:
            key, value = pair.split("=", 1)
            result[key] = value
    return result

print("<h1>Welcome on script.py</h1>")
if sys.argv[1] == "" :
    print('<form action="/script.py" method="GET">')
    print('<div>')
    print('<label for="firstname">firstname :</label>')
    print('<input name="firstname" id="firstname" placeholder="John" />')
    print('</div>')
    print('<div>')
    print('<label for="lastname">lastname : </label>')
    print('<input name="lastname" id="lastname" placeholder="Smith" />')
    print('</div>')
    print('<div>')
    print('<button>Send my greetings</button>')
    print('</div>')
    print('</form>')
else :
    data = parse_query_string(sys.argv[1])
    print("<p>Hello " + data['firstname'] + " " + data['lastname'] + "!</p>")