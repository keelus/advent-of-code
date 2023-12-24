package day24

type Hailstone struct {
	P [3]float64
	V [3]float64
}

// Formula for an object's Y position at a given x
// x(t) = X + Vx*t
// y(t) = Y + Vy*t
// So F(x) = Y + (Vy/Vx) * (x - X)
func findCollision(a, b Hailstone, minXY, maxXY float64) bool {
	aSlope := a.V[1] / a.V[0]
	bSlope := b.V[1] / b.V[0]

	if aSlope == bSlope { // Pathes are parallel
		return false
	}

	xIntersect := (a.P[0]*aSlope - b.P[0]*bSlope - a.P[1] + b.P[1]) / (aSlope - bSlope)
	yIntersect := a.P[1] + aSlope*(xIntersect-a.P[0])

	// Out of bounds
	if !(xIntersect >= minXY && xIntersect <= maxXY && yIntersect >= minXY && yIntersect <= maxXY) {
		return false
	}

	// The intersected X is from the past
	if xIntersect < a.P[0] && a.V[0] > 0 || xIntersect < b.P[0] && b.V[0] > 0 ||
		xIntersect > a.P[0] && a.V[0] < 0 || xIntersect > b.P[0] && b.V[0] < 0 {
		return false
	}

	// The intersected Y is from the past
	if yIntersect < a.P[1] && a.V[1] > 0 || yIntersect < b.P[1] && b.V[1] > 0 ||
		yIntersect > a.P[1] && a.V[1] < 0 || yIntersect > b.P[1] && b.V[1] < 0 {
		return false
	}

	return true
}
