import numpy as np
from sklearn.linear_model import LinearRegression

# Synthetic dataset: feature (x) and target (y)
X = np.array([[1.0], [2.0], [3.0], [4.0], [5.0]])
y = np.array([2.1, 4.2, 6.1, 8.3, 10.0])

# Train linear regression model
model = LinearRegression().fit(X, y)

# Predict on new data
prediction = model.predict(np.array([[6.0]]))
print("Prediction for x=6:", prediction[0])

# Print model parameters
print("Intercept:", model.intercept_)
print("Weights:", model.coef_)