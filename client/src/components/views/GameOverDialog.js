import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class GameOverDialog extends Component {
  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div className='dialog'>
            <h1>You {state.game.current_player === 0 ? 'lost' : 'won'}!</h1>
            <p>Why not play another one?</p>
            <button className='btn' onClick={mutations.startNewGame}>Start New Game</button>
          </div>
        )}
      </AppConsumer>
    )
  }
}

GameOverDialog.contextType = AppContext
