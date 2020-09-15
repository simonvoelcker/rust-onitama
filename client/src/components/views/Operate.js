import React, { Component } from 'react'

import Field from '../panels/Field'
import { AppConsumer, AppContext } from '../../context/AppContext'
import StandardButton from '../panels/StandardButton'

export default class Operate extends Component {
  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div className='operate'>
            {state.gameId === null
            ? <StandardButton onClick={() => mutations.startNewGame()}>New Game</StandardButton>
            : <Field />
            }
          </div>
        )}
      </AppConsumer>
    )
  }
}

Operate.contextType = AppContext
