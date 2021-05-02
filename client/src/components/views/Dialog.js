import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'
import StandardButton from '../panels/StandardButton'

export default class Dialog extends Component {
  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div>
            Welcome!
            This is a simple game called Onitama. I did not invent it, I merely implemented this online version of it as s hobby project.
            How to play:
            You are given control over a master of martial arts and four of his appretices. Your goal is to either beat the enemy master with any of your pieces or to reach the top middle square with your own master. You move according to the movement cards presented on your player tableau: Select one of your pieces, a movement card and a target square. The target square must be vacant or occupied by an enemy piece. After your turn, you lose the movement card he used but get the one that your opponent had used in his last turn. The five movement cards are randomly selected from sixteen existing ones, each representing a style of Kung Fu.
            <StandardButton onClick={() => mutations.startNewGame()}>New Game</StandardButton>
          </div>
        )}
      </AppConsumer>
    )
  }
}

Dialog.contextType = AppContext
