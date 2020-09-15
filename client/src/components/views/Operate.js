import React, { Component } from 'react'

import Field from '../panels/Field'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class Operate extends Component {
  render () {
    return (
      <AppConsumer>
        {({ store, mutations }) => (
          <div className='operate'>
            <Field />
          </div>
        )}
      </AppConsumer>
    )
  }
}

Operate.contextType = AppContext
