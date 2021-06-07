"""Blueprint and routes for employment section of web app."""

from flask import Blueprint, render_template

employment = Blueprint('employment', __name__)


@employment.route('/nationwide')
def nationwide():
    """Route for Nationwide employment page."""

    return render_template('employment/nationwide.html')


@employment.route('/cscc')
def cscc():
    """Route for CSCC employment page."""

    return render_template('employment/cscc.html')


@employment.route('/us-army')
def us_army():
    """Route for U.S. Army employment page."""

    return render_template('employment/us-army.html')
