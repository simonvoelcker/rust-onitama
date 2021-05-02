import React, { Component } from 'react'

import Field from '../panels/Field'
import { AppConsumer, AppContext } from '../../context/AppContext'
import Dialog from './Dialog'

export default class Game extends Component {
  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div>
            {state.game === null
            ? <Dialog />
            : <Field />
            }
          </div>
        )}
      </AppConsumer>
    )
  }
}

Game.contextType = AppContext
