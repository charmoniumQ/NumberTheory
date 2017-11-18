#include <vector>


template <typename Int>
class char_tri<Int> {
  // computes and caches the charactersitic triangle on demand
  // so that other computations can be computed quickly
public:
  char_tri();
  void calculate(Int rows);
  bool k_divides(Int a, Int b, Int k);
  bool k_coprime(Int a, Int b, Int k);
  Int k_gcd(Int a, Int b, Int k);

private:
  std::vector<Int> triangle;
  Int n_rows;
  // vector was used here because it has fast random-access (like arrays)
  // but it can be resized on demand

};


  // Here instead of a ragged 2d-array
  // I am going to say triangle[k*(k+1)/2 + n] is the nth col of the kth row.
template <typename Int>
inline pos<Int>(Int row, Int col) { return row * (row + 1) / 2 + col}

// builds the characteristic triangle up to new_n_rows
void char_tri<Int>::calculate(Int new_n_rows) {
  triangle.resize(rows * (rows + 1) / 2);

  for (Int row = n_rows; row <= new_n_rows; ++row) {
	triangle[pos(row, 0)] = 1;
	triangle[pos(row, row)] = 1;

	for (Int a = 1; a < row; ++a) {
	  bool inf_div = (a & row) == a;
	  Int lb = inf_div ? 1 : 2;
	  Int ub = b;
	  if (ub % 2 != int(inf_div)) {
		ub--;
	  }

	  // binary search
	  while (lb != ub) {
		int m = inf_div ? (lb + ub + 2) / 4 * 2 - 1 : (lb + ub) / 4 * 2;
		if (k_divides(a, b, m)) {
		  ub = m;
		} else {
		  lb = m + 2;
		}
	  }

	  triangle[pos(row, a)] = lb;
	}
  }

  n_rows = new_n_rows;
}

// whether p^a k-ary divides p^b
bool char_tri<Int>::k_divides(Int a, Int b, Int k) {
  reutrn a <= b and k_coprime(a, b-a, k-1);
}

// whether p^a is k-ary coprime to p^b
bool char_tri<Int>::k_coprime(Int a, Int b, Int k) {
  return k_gcd(a, b, k) == 1;
}

template <typename T>
class sorted_vector : public vector<T>{}

Int char_tri<Int>::k_gcd(Int a, Int b, Int k) {
  
  for (Int i = 1; i <= min(a, b); i++) {
	if (k_divides(a, i, k - 1) && k_divides(b, i, k - 1))
	  return false;
  }
  return true;
}

/* vector<Int> char_tri<Int>::k_gcd(Int a, Int b, Int k) { */
/*   for (int i) */
/* } */

// whether the number k1 on the characteristic triangle indicates that d k2-ary divides n
bool char_tri<Int>::order (Int row, Int i, Int k2) {
  Int k1 = traingle[pos(row, i)];
  if (k1 % 2 == 0)
	return k2 % 2 == 0 && k1 > k2;
  else
	return k2 % 2 == 0 || k1 <= k2;
}

char_tri<Int>::char_tri()
: triangle (0), n_rows (0) { }
