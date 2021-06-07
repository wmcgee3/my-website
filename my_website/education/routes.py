"""Blueprint and routes for education section of web app."""

from flask import Blueprint, render_template

education = Blueprint('education', __name__, url_prefix='/education')


@education.route('/cscc')
def cscc():
    """Route for CSCC education."""

    return render_template('education/cscc.html')


@education.route('/dli-flc')
def dli_flc():
    """Route for DLI-FLC education."""

    return render_template('education/dli-flc.html')
