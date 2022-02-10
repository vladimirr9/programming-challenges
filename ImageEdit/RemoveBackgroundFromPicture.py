import numpy as np
import cv2
import matplotlib
import matplotlib.pyplot as plt

img_name = "3"

img = cv2.imread(img_name + '.jpg')
img = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)

img = img.astype(np.uint8)

image_ada_bin = cv2.adaptiveThreshold(img, 255, cv2.ADAPTIVE_THRESH_MEAN_C, cv2.THRESH_BINARY, 165, 30)
plt.figure()
plt.imshow(image_ada_bin, 'gray')

plt.show()

cv2.imwrite(img_name + "fixed.jpg", image_ada_bin)
