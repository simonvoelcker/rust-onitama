import React, { Component } from 'react'

import Field from '../panels/Field'
import { AppConsumer, AppContext } from '../../context/AppContext'
import StandardButton from '../panels/StandardButton'

export default class Game extends Component {
  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div>
            {state.game === null
            ? <StandardButton onClick={() => mutations.startNewGame()}>New Game</StandardButton>
            : <Field />
            }
          </div>
        )}
      </AppConsumer>
    )
  }
}

Game.contextType = AppContext
