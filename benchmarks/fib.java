
class fib {

	static int fib(int n) {
		if(n < 2) {
			return n;
		} else {
			return fib(n-1) + fib(n-2);
		}
	}

	public static void main(String...args) {
		System.out.println("Fib: "+fib(30));
	}
}