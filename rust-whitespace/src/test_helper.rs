pub fn prepare_example(input: &str) -> String {
  input.replace("\n", "")
  .replace("[Space]", " ")
  .replace("[Tab]", "\t")
  .replace("[LF]", "\n")
}
