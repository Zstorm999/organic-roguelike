# credits to Iddo Hanniel on Stack Overflow
# https://stackoverflow.com/a/50159452

from scipy.spatial import Delaunay
import numpy as np


def alpha_shape(points, alpha, only_outer=True):
    """
    Compute the alpha shape (concave hull) of a set of points.
    :param points: np.array of shape (n,2) points.
    :param alpha: alpha value.
    :param only_outer: boolean value to specify if we keep only the outer border
    or also inner edges.
    :return: set of (i,j) pairs representing edges of the alpha-shape. (i,j) are
    the indices in the points array.
    """
    assert points.shape[0] > 3, "Need at least four points"

    def add_edge(edges, i, j):
        """
        Add a line between the i-th and j-th points,
        if not in the list already
        """
        if (i, j) in edges or (j, i) in edges:
            # already added
            assert (j, i) in edges, "Can't go twice over same directed edge right?"
            edges.remove((j, i))
            print("Removed")
            return
        edges.add((i, j))

    tri = Delaunay(points)
    edges = set()
    # Loop over triangles:
    # ia, ib, ic = indices of corner points of the triangle
    for ia, ib, ic in tri.simplices:
        pa = points[ia]
        pb = points[ib]
        pc = points[ic]
        # Computing radius of triangle circumcircle
        # www.mathalino.com/reviewer/derivation-of-formulas/derivation-of-formula-for-radius-of-circumcircle
        a = np.sqrt((pa[0] - pb[0]) ** 2 + (pa[1] - pb[1]) ** 2)
        b = np.sqrt((pb[0] - pc[0]) ** 2 + (pb[1] - pc[1]) ** 2)
        c = np.sqrt((pc[0] - pa[0]) ** 2 + (pc[1] - pa[1]) ** 2)
        s = (a + b + c) / 2.0
        area = np.sqrt(s * (s - a) * (s - b) * (s - c))
        circum_r = a * b * c / (4.0 * area)
        if circum_r < alpha:
            add_edge(edges, ia, ib)
            add_edge(edges, ib, ic)
            add_edge(edges, ic, ia)
    return edges


from matplotlib.pyplot import *

from time import perf_counter

# Constructing the input point data
# np.random.seed(0)

nb_points = 300
x = 3.0 * np.random.rand(nb_points)
y = 3.0 * np.random.rand(nb_points)
inside = (
    (x ** 2 + y ** 2 > 1.0)
    & ((x - 3) ** 2 + y ** 2 > 1.0)
    & ((x - 1.5) ** 2 + y ** 2 > 0.09)
)
points = np.vstack([x[inside], y[inside]]).T

points = np.vstack([x, y]).T

now = perf_counter()

# Computing the alpha shape
edges = alpha_shape(points, alpha=1, only_outer=False)

print(f"{perf_counter() - now}")
print(len(edges))

# Plotting the output
figure()
axis("equal")
plot(points[:, 0], points[:, 1], ".")
for i, j in edges:
    plot(points[[i, j], 0], points[[i, j], 1])
show()
