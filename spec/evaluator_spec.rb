require 'spec_helper'

describe "Evaluator" do
  def run(src)
    ast = Shiika::Parser.new.parse(src)
    program = ast.to_program
    return Shiika::Evaluator.new.run(program)
  end

  def sk_int(n)
    Shiika::Evaluator::SkObj.new(Shiika::Type::TyRaw['Int'], {'@rb_val' => n})
  end

  describe 'generic class' do
    it 'Pair example' do
      src = <<~EOD
        class Pair<S, T>
          def initialize(@a: S, @b: T)
          end

          def fst -> S; @a; end
          def snd -> T; @b; end
        end
        Pair<Int, Bool>.new(1, true).fst
# TODO
#        class IntBoolPair extends Pair<Int, Bool>
#        end
#        IntBoolPair.new(1, true).fst
      EOD
      expect(run(src)).to eq(sk_int(1))
    end
  end

  it 'instance variable' do
    src = <<~EOD
      class Adder
        def initialize(@x: Int, @y: Int)
        end

        def sum() -> Int
          @x + @y
        end
      end
      Adder.new(1, 2).sum
    EOD
    expect(run(src)).to eq(sk_int(3))
  end

  describe 'method' do
    it 'vararg' do
      src = <<~EOD
        class A
          def self.first(*b: [Int]) -> Int
            b.first
          end
        end
        A.first(1,2,3)
      EOD
      expect(run(src)).to eq(sk_int(1))
    end
  end

  it 'instance generation' do
    src = <<~EOD
      class A
        def foo -> Int
          2
        end
      end
      A.new.foo
    EOD
    expect(run(src)).to eq(sk_int(2))
  end

  it 'class method invocation' do
    src = <<~EOD
      class A
        def self.foo(x: Int) -> Int
          x
        end
      end
      A.foo(1)
    EOD
    expect(run(src)).to eq(sk_int(1))
  end

  it 'calling Shiika method from stdlib function' do
    expect(run("1.tmp")).to eq(sk_int(1))
  end

  it 'stdlib method invocation' do
    expect(run("1 + 1")).to eq(sk_int(2))
  end

  it 'local variable' do
    expect(run("a = 1; a")).to eq(sk_int(1))
  end

  it 'if' do
    expect(run("if true; 1; else 2; end")).to eq(sk_int(1))
  end

  describe 'literal' do
    it 'array' do
      expect(run("[1, 2, 3].first")).to eq(sk_int(1))
    end

    it 'number' do
      expect(run("123")).to eq(sk_int(123))
    end
  end
end
