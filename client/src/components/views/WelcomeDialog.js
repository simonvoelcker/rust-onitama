import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class WelcomeDialog extends Component {
  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div className='dialog-wrapper'>
            <div className='dialog'>
                <h1>Welcome!</h1>
                <p>This is a simple game called Onitama. I did not invent it, I merely implemented this online version of it as s hobby project.</p>
                <h2>How to play:</h2>
                <p>You are given control over a Kung Fu master and his apprentices.</p>
                <p><b>Your goal is to either beat the enemy master with any of your pieces OR to reach the top middle square with your own master.</b></p>
                <p>You move according to the <i>movement cards</i> presented on your player tableau:</p>
                <p><b>Select one of your pieces, a movement card and a target square. The movement cards allow for different moves, as indicated by grey squares.</b></p>
                <p>The target square must not be occupied by one of your own pieces already.
                After your turn, you lose the movement card you used but get the one that your opponent had used in his last turn.
                The five movement cards are randomly selected from sixteen existing ones, each representing a different style of Kung Fu.</p>
                <p><b>Enjoy!</b></p>
                <button className='btn' onClick={mutations.startNewGame}>Start Game</button>
            </div>
          </div>
        )}
      </AppConsumer>
    )
  }
}

WelcomeDialog.contextType = AppContext
