import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class GameOverDialog extends Component {

  getRandomMessage(win) {
    let messages = win ? [
        "The enemy didn't stand a chance, if we're honest.",
        "You beat the computer on its own turf!",
        "Bruce Lee himself couldn't have done it any better.",
        "And what a finishing move it was. I'll never forget.",
        "The enemy is probably scared of you now.",
        "You came, you saw, you won.",
        "Flawless execution, that's for sure.",
        "That look on the enemy master's face, priceless!",
        "I'm impressed. Can you do it again?",
        "Nobody messes with you.",
    ] : [
        "He probably only won because he was better than you.",
        "That was super unfair. He had all the good cards.",
        "But you fought well... At least I think you did.",
        "Well, at least you tried. Try again, though?",
        "You were definitely ahead in the first half.",
        "I did not see this coming, honestly.",
        "You cannot give up halfway through. This is Kung Fu.",
        "Your effort will be remembered, though.",
        "It happened so quickly! How did he do that?",
        "You learned something, that is what counts.",
    ]
    let randomIndex = Math.floor(Math.random() * messages.length)
    return messages[randomIndex]
  }

  render () {
    let win = this.context.state.game.current_player === 1
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div className='dialog-wrapper'>
            <div className='dialog'>
              <h1>You {win ? 'won' : 'lost'}!</h1>
              <p>{this.getRandomMessage(win)}</p>
              <button className='btn' onClick={mutations.startNewGame}>Start new game</button>
            </div>
          </div>
        )}
      </AppConsumer>
    )
  }
}

GameOverDialog.contextType = AppContext
