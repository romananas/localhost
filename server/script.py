import sys
import json

print("<h1>Welcome on script.py</h1>")

if len(sys.argv) < 2 or sys.argv[1].strip() == "":
    print('<form action="/script.py" method="POST" enctype="application/json">')
    print('<div>')
    print('<label for="firstname">firstname :</label>')
    print('<input name="firstname" id="firstname" placeholder="John" />')
    print('</div>')
    print('<div>')
    print('<label for="lastname">lastname : </label>')
    print('<input name="lastname" id="lastname" placeholder="Smith" />')
    print('</div>')
    print('<div>')
    print('<button type="submit">Send my greetings</button>')
    print('</div>')
    print('</form>')
else:
    try:
        data = json.loads(sys.argv[1])
        firstname = data.get("firstname", "Unknown")
        lastname = data.get("lastname", "")
        print(f"<p>Hello {firstname} {lastname}!</p>")
    except json.JSONDecodeError:
        print("<p>Invalid JSON received.</p>")
