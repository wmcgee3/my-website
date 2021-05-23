import flask

def create_app():
    app = flask.Flask(__name__)

    from my_website.main.routes import main

    app.register_blueprint(main)

    return app
