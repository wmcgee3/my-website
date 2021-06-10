from waitress import serve

from my_website import create_app

app = create_app()

serve(app, host='127.0.0.1', port=8080)
