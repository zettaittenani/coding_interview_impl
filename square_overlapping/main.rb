# エンジニアとしての生き方 (著: 中島聡) より

def square_overlapping?(a_x_l, a_x_g, a_y_l, a_y_g, b_x_l, b_x_g, b_y_l, b_y_g)
  if a_x_g < b_x_l
    false
  elsif b_x_g < a_x_l
    false
  elsif a_y_g < b_y_l
    false
  elsif b_y_g < a_y_l
    false
  else
    true
  end
end

# false を期待する
# 被っていない
test1 = !square_overlapping?(0, 1, 0, 1, 2, 3, 2, 3)

# true を期待する
# 点で被っている (左下)
test2 = square_overlapping?(0, 1, 0, 1, 1, 2, 1, 2)

# true を期待する
# 線で被っている (下)
test3 = square_overlapping?(0, 1, 0, 1, 0, 1, 1, 2)

# true を期待する
# 割と被っている
test4 = square_overlapping?(0, 3, 0, 3, 1, 4, 1, 4)

# true を期待する
# 片方を覆っている
test5 = square_overlapping?(0, 3, 0, 3, 1, 2, 1, 2)

p [test1, test2, test3, test4, test5].all?
