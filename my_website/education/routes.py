from flask import Blueprint, render_template

education = Blueprint('education', __name__, url_prefix='/education')


@education.route('/cscc')
def cscc():
    return render_template('education/cscc.html')


@education.route('/dli-flc')
def dli_flc():
    return render_template('education/dli-flc.html')
