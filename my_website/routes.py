"""Base blueprint and routes for web application."""

from flask import Blueprint, render_template

from my_website.education.routes import education
from my_website.employment.routes import employment

main = Blueprint('main', __name__)

main.register_blueprint(education)
main.register_blueprint(employment)


@main.route('/')
def home():
    """Route for home page."""

    return render_template('home.html')


@main.route('/contact')
def contact():
    """Route for contact page."""

    return render_template('contact.html')
