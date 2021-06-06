from flask import Blueprint, render_template

employment = Blueprint('employment', __name__)


@employment.route('/nationwide')
def nationwide():
    return render_template('employment/nationwide.html')


@employment.route('/cscc')
def cscc():
    return render_template('employment/cscc.html')


@employment.route('/us-army')
def us_army():
    return render_template('employment/us-army.html')
