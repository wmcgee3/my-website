"""Module to create Flask application."""

from flask import Flask

def create_app():
    """Create and return Flask application."""

    # pylint: disable=import-outside-toplevel

    app = Flask(__name__)

    from my_website.routes import main

    app.register_blueprint(main)

    return app
