import collections

Card = collections.namedtuple('Card', ['rank', 'suit'])


class FrenchDeck:
	ranks = [str(n) for n in range(2, 11)] + list('JQKA')
	suits = 'spades diamonds clubs hearts'.split()

	def __init__(self):
		self._cards = [Card(rank, suit) for suit in self.suits for rank in self.ranks]

	def __len__(self):
		return len(self._cards)

	def __getitem__(self, position: int) -> Card:
		return self._cards[position]


suit_values = dict(spades=3, hearts=2, diamonds=1, clubs=0)


def spades_high(card: Card):
	rank_value = FrenchDeck.ranks.index(card.rank)
	return rank_value * len(suit_values) + suit_values[card.suit]


def french_deck_example():
	deck = FrenchDeck()

	card = Card('Q', 'hearts')
	print(f'{card} is in Deck? {card in deck}')

	print(suit_values, len(suit_values))

	ordered_card = sorted(deck, key=spades_high)

	print(f'first card {ordered_card[0]}, last card {ordered_card[-1]}')
