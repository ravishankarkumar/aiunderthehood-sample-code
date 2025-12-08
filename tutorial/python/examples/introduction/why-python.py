from sklearn.linear_model import LinearRegression
import numpy as np

# Training data
X = np.array([[1], [2], [3], [4]])
y = np.array([2, 4, 6, 8])

# Train model
model = LinearRegression()
model.fit(X, y)

print("Prediction for 5:", model.predict([[5]]))
# Prediction for 5: [10.]