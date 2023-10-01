import Mathlib.Data.Multiset.Basic
import Mathlib.Data.Rat.Basic
import Mathlib.Data.List.Sort
import Mathlib.Tactic

set_option autoImplicit false

inductive Makes : ℚ → Multiset ℕ → Prop
  | base (n : ℕ) : Makes n [n]
  | add : {n m : ℚ} → {k l : Multiset ℕ} → Makes n k → Makes m l → Makes (n + m) (k + l) 
  | sub : {n m : ℚ} → {k l : Multiset ℕ} → Makes n k → Makes m l → Makes (n - m) (k + l) 
  | mul : {n m : ℚ} → {k l : Multiset ℕ} → Makes n k → Makes m l → Makes (n * m) (k + l) 
  | div : {n m : ℚ} → (m ≠ 0) → {k l : Multiset ℕ} → Makes n k → Makes m l → Makes (n / m) (k + l)

infixl:65 " +' " => Makes.add
infixl:65 " -' " => Makes.sub
infixl:70 " *' " => Makes.mul
infixl:70 " /' " => Makes.div (by norm_num)
notation:1024 "$" arg:1024 => Makes.base arg

theorem case1125 : Makes 1 [1, 1, 2, 5] := by convert $5 -' $2 -' $1 -' $1; simp
theorem case1126 : Makes 1 [1, 1, 2, 6] := by convert $6 /' $2 -' $1 -' $1; simp
theorem case1456 : Makes 1 [1, 4, 5, 6] := by convert ($6 +' $4) /' $5 -' $1; simp
theorem case2225 : Makes 1 [2, 2, 2, 5] := by convert $2 +' $2 +' $2 -' $5
theorem case2226 : Makes 1 [2, 2, 2, 6] := by convert $2 +' $2 -' ($6 /' $2); simp
theorem case2555 : Makes 1 [2, 5, 5, 5] := by convert ($5 +' $5) /' ($2 *' $5); simp
theorem case2556 : Makes 1 [2, 5, 5, 6] := by convert $6 +' $5 -' $2 *' $5; simp
theorem case2566 : Makes 1 [2, 5, 6, 6] := by convert $2 *' $6 -' ($5 +' $6); simp
theorem case2666 : Makes 1 [2, 6, 6, 6] := by convert ($6 +' $6) /' ($2 *' $6); simp
theorem case3666 : Makes 1 [3, 6, 6, 6] := by convert $6 /' $3 -' $6 /' $6; simp

theorem List.length_eq_four (l : List ℕ) (_ : List.length l = 4) : ∃ a b c d, l = [a, b, c, d] :=
  let [a, b, c, d] := l; ⟨a, b, c, d, rfl⟩

theorem card_eq_four (l : Multiset ℕ) : (h : Multiset.card l = 4) → 
    ∃ (a b c d : ℕ), l = [a, b, c, d] ∧ List.Sorted (· ≤ ·) [a, b, c, d] := by
  refine Quot.inductionOn l ?_
  intro l h
  let l' := List.insertionSort (· ≤ ·) l
  have hl' : l' ~ l := List.perm_insertionSort (· ≤ ·) l
  have hl'' : List.length l' = 4 := hl'.length_eq.trans h
  obtain ⟨a, b, c, d, h⟩ := List.length_eq_four _ hl''
  refine ⟨a, b, c, d, ⟨?_, ?_⟩⟩
  · simpa [← h] using hl'.symm
  · rw [← h]
    exact List.sorted_insertionSort _ l

theorem annihilate₁ {b d : ℕ} : Makes 1 [1, b, b, d] := by
  convert $1 +' (Makes.base b -' Makes.base b) *' Makes.base d
  simp

theorem annihilate₂ {b c : ℕ} : Makes 1 [1, b, c, c] := by
  convert $1 +' (Makes.base c -' Makes.base c) *' Makes.base b
  · simp
  · erw [Multiset.coe_eq_coe]
    simp
    refine (List.Perm.swap _ _ _).trans ?_
    simp
    exact List.Perm.swap _ _ _

theorem lemma₁ {a b c d : ℕ} (h₁ : Makes 1 [a, b] ∨ Makes 2 [a, b]) (h₂ : Makes 1 [c, d] ∨ Makes 2 [c, d]) :
    Makes 1 [a, b, c, d] := by
  rcases h₁ with (h₁|h₁)
  · rcases h₂ with (h₂|h₂)
    · convert h₁ *' h₂
    · convert h₂ -' h₁
      erw [Multiset.coe_eq_coe]
      refine List.Perm.trans ?_ List.perm_append_comm
      simp
  · rcases h₂ with (h₂|h₂)
    · convert h₁ -' h₂
    · convert h₁ /' h₂

theorem lemma₂_aux {a b : ℕ} (hb : a ≠ 0) (h' : a ≤ b) (h : b - a ≤ 2) : Makes 1 [a, b] ∨ Makes 2 [a, b] := by
  interval_cases hab : b - a
  · convert Or.inl (Makes.div _ (Makes.base a) (Makes.base b))
    · obtain rfl : a = b := by
        rw [Nat.sub_eq_iff_eq_add h'] at hab
        rw [hab, zero_add]
      apply Eq.symm
      rw [div_eq_one_iff_eq]
      exact Nat.cast_ne_zero.2 hb
    · aesop
  · convert Or.inl ((Makes.base b) -' (Makes.base a))
    · apply Eq.symm
      assumption_mod_cast
    · erw [Multiset.coe_eq_coe]
      exact List.Perm.swap _ _ _ 
  · convert Or.inr ((Makes.base b) -' (Makes.base a))
    · apply Eq.symm
      assumption_mod_cast
    · erw [Multiset.coe_eq_coe]
      exact List.Perm.swap _ _ _ 

theorem lemma₂ {a b c d : ℕ} (ha : a ≠ 0) (hc : c ≠ 0) (hab : a ≤ b) (hcd : c ≤ d)
    (h₁ : b - a ≤ 2) (h₂ : d - c ≤ 2) : Makes 1 [a, b, c, d] :=
  lemma₁ (lemma₂_aux ha hab h₁) (lemma₂_aux hc hcd h₂)

theorem lemma₃ {a b c d : ℕ} (ha : a ≠ 0) (hab : a ≤ b) (hab' : b - a ≤ 2) (hc : 3 ≤ c)
    (hcd : c ≤ d) (hd : d ≤ 6) : Makes 1 [a, b, c, d] := by
  rcases eq_or_ne c 3 with (rfl|hc')
  · rcases eq_or_ne d 6 with (rfl|hd')
    · refine lemma₁ (lemma₂_aux ha hab hab') (Or.inr ?_)
      convert $6 /' $3
      simp
    · apply lemma₂ ha (by linarith) hab hcd hab'
      interval_cases d
      all_goals aesop
  · apply lemma₂ ha (by linarith) hab hcd hab'
    interval_cases d
    all_goals { 
      interval_cases c
      all_goals aesop }

theorem goal (l : Multiset ℕ) (h₁ : Multiset.card l = 4) (h₂ : ∀ x ∈ l, 1 ≤ x ∧ x ≤ 6) : Makes 1 l := by
  obtain ⟨a, b, c, d, ⟨rfl, h⟩⟩ := card_eq_four _ h₁
  clear h₁
  simp at h
  rcases h with ⟨⟨hab, hac, had⟩, ⟨hbc, hbd⟩, hcd⟩
  simp at h₂
  rcases h₂ with ⟨⟨ha₁, ha₂⟩, ⟨hb₁, hb₂⟩, ⟨hc₁, hc₂⟩, ⟨hd₁, hd₂⟩⟩
  by_cases ha : 4 ≤ a
  · apply lemma₂ (Nat.pos_iff_ne_zero.1 ha₁) (Nat.pos_iff_ne_zero.1 hc₁) hab hcd
    · rw [Nat.sub_le_iff_le_add]
      calc b ≤ 6 := hb₂
        _ = 2 + 4 := rfl
        _ ≤ 2 + a := Nat.add_le_add_left ha _
    · rw [Nat.sub_le_iff_le_add]
      calc d ≤ 6 := hd₂
        _ = 2 + 4 := rfl
        _ ≤ 2 + a := Nat.add_le_add_left ha _
        _ ≤ 2 + c := Nat.add_le_add_left hac _
  · interval_cases a 
    · rcases eq_or_ne b c with (rfl|hbc')
      · exact annihilate₁
      rcases eq_or_ne c d with (rfl|hcd')
      · exact annihilate₂
      by_cases hb : b ≥ 5
      · interval_cases b
        · interval_cases c
          · contradiction
          · interval_cases d
            contradiction
        · interval_cases c
          contradiction
      · interval_cases b
        · by_cases hc : 3 ≤ c 
          · exact lemma₃ (by norm_num) (by norm_num) (by norm_num) hc hcd hd₂
          · interval_cases c
            · contradiction
            · by_cases hd : d ≤ 4
              · apply lemma₂ (by norm_num) (by norm_num) (by norm_num) hcd (by norm_num)
                rw [Nat.sub_le_iff_le_add]
                exact hd
              · interval_cases d
                exacts [case1125, case1126]
        · refine lemma₃ (by norm_num) (by norm_num) (by norm_num) ?_ hcd hd₂
          exact Nat.lt_of_le_of_ne hbc hbc'
        · exact lemma₃ (by norm_num) (by norm_num) (by norm_num) hbc hcd hd₂
        · interval_cases c
          · contradiction
          · interval_cases d
            · contradiction
            · exact case1456
          · interval_cases d
            · contradiction 
    · rcases eq_or_ne b 2 with (rfl|hb)
      · by_cases hc : 3 ≤ c
        · exact lemma₃ (by norm_num) (by norm_num) (by norm_num) hc hcd hd₂
        · interval_cases c
          by_cases hd : d ≤ 4
          · apply lemma₂ (by norm_num) (by norm_num) (by norm_num) hbd (by norm_num)
            rw [Nat.sub_le_iff_le_add]
            exact hd
          · interval_cases d
            exacts [case2225, case2226]
      · by_cases hb' : b ≤ 4
        · apply lemma₃ (by norm_num) hab (Nat.sub_le_iff_le_add.2 hb') _ hcd hd₂
          refine le_trans ?_ hbc
          exact Nat.lt_of_le_of_ne hab hb.symm
        · interval_cases b
          · interval_cases c
            · interval_cases d
              exacts [case2555, case2556]
            · interval_cases d
              exact case2566
          · interval_cases c
            interval_cases d
            exact case2666
    · by_cases hb : b ≤ 5
      · apply lemma₃ (by norm_num) hab _ hac hcd hd₂
        rw [Nat.sub_le_iff_le_add]
        exact hb
      · interval_cases b
        interval_cases c
        interval_cases d
        exact case3666 