import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'

export default class Field extends Component {
  constructor (props) {
    super(props)
    this.state = {
    }
  }

  render () {
    return (
      <AppConsumer>
        {({ store, mutations }) => (
          <div className='field-panel'>
          </div>
        )}
      </AppConsumer>
    )
  }
}

Field.contextType = AppContext
