package pair

import "strconv"

// A Pair is an I, J coordinate pair.
type Pair struct {
	I, J int
}

// String returns a string representation of p like "(3,4)".
func (p Pair) String() string {
	return "(" + strconv.Itoa(p.I) + "," + strconv.Itoa(p.J) + ")"
}

// Add returns the Pair p+q.
func (p Pair) Add(q Pair) Pair {
	return Pair{p.I + q.I, p.J + q.J}
}

// Sub returns the Pair p-q.
func (p Pair) Sub(q Pair) Pair {
	return Pair{p.I - q.I, p.J - q.J}
}

// Mul returns the Pair p*k.
func (p Pair) Mul(k int) Pair {
	return Pair{p.I * k, p.J * k}
}

// Div returns the Pair p/k.
func (p Pair) Div(k int) Pair {
	return Pair{p.I / k, p.J / k}
}

// Eq reports whether p and q are equal.
func (p Pair) Eq(q Pair) bool {
	return p.I == q.I && p.J == q.J
}

// Zero reports whether the Pair p is (0, 0)
func (p Pair) Zero() bool {
	return p.I == 0 && p.J == 0
}

// Clone returns a copy of the Pair p.
func (p Pair) Copy() Pair {
	return Pair{p.I, p.J}
}

// New is shorthand for Pair{I, J}.
func New(I, J int) Pair {
	return Pair{I, J}
}

// Zero is shorthand for Pair{0, 0}.
func Zero() Pair {
	return Pair{}
}
