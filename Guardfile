guard :shell, all_on_start: true do
  watch /.*\.l?rs$/ do |m|
    puts "\n\n\nCompiling..."
    `rustc -g #{m[0]} && ./#{m[0][0..-4]} && echo "Compiled!"`
  end
end

# vim:ft=ruby