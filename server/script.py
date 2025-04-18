import sys
import json

html_template = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Profile</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            background-color: #f4f4f4;
            padding: 20px;
            color: #333;
        }}
        .container {{
            max-width: 400px;
            margin: auto;
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 0 10px rgba(0,0,0,0.1);
            text-align: center;
        }}
        img {{
            border-radius: 50%;
            max-width: 150px;
            height: auto;
            margin-bottom: 15px;
        }}
        h1 {{
            color: #2c3e50;
        }}
    </style>
</head>
<body>
    <div class="container">
        {content}
    </div>
</body>
</html>
"""

def render_content(firstname, lastname, avatar_url):
    return f"""
    <h1>Bienvenue {firstname} {lastname}!</h1>
    <img src="{avatar_url}" alt="Avatar de {firstname}">
    """

def render_error(msg):
    return f"<h1>Erreur :</h1><p>{msg}</p>"

# Logique principale
if len(sys.argv) < 2 or sys.argv[1].strip() == "":
    content = render_error("Données non reçues")
else:
    try:
        data = json.loads(sys.argv[1])
        firstname = data.get("firstname", "Inconnu")
        lastname = data.get("lastname", "")
        avatar = data.get("avatar", "")
        if avatar:
            content = render_content(firstname, lastname, avatar)
        else:
            content = render_content(firstname, lastname, "https://via.placeholder.com/150")
    except json.JSONDecodeError:
        content = render_error("JSON invalide")

print(html_template.format(content=content))
