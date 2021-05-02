import React, { Component } from 'react'

import Field from '../panels/Field'
import { AppConsumer, AppContext } from '../../context/AppContext'
import WelcomeDialog from './WelcomeDialog'
import GameOverDialog from './GameOverDialog'

export default class Game extends Component {
  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div>
            {state.game === null && <WelcomeDialog />}
            {state.game !== null && <Field />}
            {state.game !== null && state.options && state.options.length === 0 && <GameOverDialog />}
          </div>
        )}
      </AppConsumer>
    )
  }
}

Game.contextType = AppContext
