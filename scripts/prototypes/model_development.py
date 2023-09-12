# Full model development process
#
# This file contains
#

from stability import (
    make_bootstrapped_resamples,
    predict_bootstrapped_proba,
    plot_instability,
)
from fit import fit_logistic_regression
from calibration import (
    get_bootstrapped_calibration,
    plot_calibration_curves,
    plot_prediction_distribution,
)
from roc import get_bootstrapped_roc, get_bootstrapped_auc, plot_roc_curves

from sklearn.datasets import make_classification
from sklearn.model_selection import train_test_split

import matplotlib.pyplot as plt
import pandas as pd
import save_datasets as ds

import seaborn as sns

# Example data for now. X is the feature matrix (each column is a feature)
# and y is the classification outcome (1 for event occurred). Both must have
# the same number of rows (the number of samples). Both are numerical,
# (X is floating point and y is integer).
#X, y = make_classification(
#    n_samples=1000, n_features=20, n_informative=2, n_redundant=2, random_state=42
#)

# Get real dataset. The bleeding outcome column is called bleeding_al_ani_outcome
# (becomes y) and the other columns except the *_outcome ones become predictors
outcome_column = "bleeding_al_ani_outcome"
non_predictors = ["bleeding_al_ani_outcome", "acs_bezin_outcome"]
dataset = ds.load_dataset("hes_episodes_dataset")
X = dataset.drop(columns = non_predictors).to_numpy()
y = dataset[outcome_column].to_numpy()
pd.set_option('display.max_rows', 500)
print(dataset[["bleeding_al_ani_outcome", "bleeding_al_ani_before", "bleeding_adaptt_before", "bleeding_cadth_before"]].head(100))
print(dataset.columns)
#exit()

print(X.shape)
print(dataset.shape)

print(dataset.isna().sum())

# Calculate correlations
corr = dataset.corr()
print(corr)
sns.heatmap(corr)
plt.tight_layout()
plt.show()
exit()

# Split (X,y) into a testing set (X_test, y_test), which is not used for
# any model training, and a training set (X0,y0), which is used to develop
# the model. Later, (X0,y0) is resampled to generate N additional training
# sets (Xn,yn) which are used to assess the stability of the developed model
# (see stability.py). All models are tested using the testing set.
train_test_split_seed = 22
test_set_proportion = 0.25
X0_train, X_test, y0_train, y_test = train_test_split(
    X, y, test_size=test_set_proportion, random_state=train_test_split_seed
)

# Develop a single model from the training set (X0_train, y0_train),
# using any method (e.g. including cross validation and hyperparameter
# tuning) using training set data. This is referred to as D in
# stability.py.
print("Fitting the main model")
M0 = fit_logistic_regression(X0_train, y0_train)
print(M0["logisticregression"].coef_)


print(y_test.mean())
exit()

from sklearn.metrics import RocCurveDisplay

RocCurveDisplay.from_estimator(M0, X_test, y_test)
plt.show()
exit()

# For the purpose of assessing model stability, obtain bootstrap
# resamples (Xn_train, yn_train) from the training set (X0, y0).
print("Creating bootstrap resamples of X0 for stability checking")
Xn_train, yn_train = make_bootstrapped_resamples(X0_train, y0_train, M=200)

# Develop all the bootstrap models to compare with the model-under-test M0
print("Fitting bootstrapped models")
Mn = [fit_logistic_regression(X, y) for (X, y) in zip(Xn_train, yn_train)]

# First columns is the probability of 1 in y_test from M0; other columns
# are the same for the N bootstrapped models Mn.
probs = predict_bootstrapped_proba(M0, Mn, X_test)

# Plot the basic instability curve
fig, ax = plt.subplots()
plot_instability(ax, probs)
plt.show()

# Get the bootstrapped calibration curves
calibration_curves = get_bootstrapped_calibration(probs, y_test, n_bins=10)

# Plot the calibration-stability plots
fig, ax = plt.subplots(2, 1)
plot_calibration_curves(ax[0], calibration_curves)
# Plot the distribution of predicted probabilities, also
# showing distribution stability (over the bootstrapped models)
# as error bars on each bin height
plot_prediction_distribution(ax[1], probs, n_bins=10)
plt.show()

# Get the bootstrapped ROC curves
roc_curves = get_bootstrapped_roc(probs, y_test)
roc_auc = get_bootstrapped_auc(probs, y_test)

# Plot the ROC-stability curves
fig, ax = plt.subplots()
plot_roc_curves(ax, roc_curves, roc_auc)
plt.show()
